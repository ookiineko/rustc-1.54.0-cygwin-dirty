use crate::{basic_manifest, project};
use filetime::{self, FileTime};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

static CARGO_INTEGRATION_TEST_DIR: &str = "cit";

lazy_static! {
    // TODO: Use `SyncOnceCell` when stable
    static ref GLOBAL_ROOT: Mutex<Option<PathBuf>> = Mutex::new(None);

    static ref TEST_ROOTS: Mutex<HashMap<String, PathBuf>> = Default::default();
}

/// This is used when running cargo is pre-CARGO_TARGET_TMPDIR
/// TODO: Remove when CARGO_TARGET_TMPDIR grows old enough.
fn global_root_legacy() -> PathBuf {
    let mut path = t!(env::current_exe());
    path.pop(); // chop off exe name
    path.pop(); // chop off "deps"
    path.push("tmp");
    path.mkdir_p();
    path
}

fn set_global_root(tmp_dir: Option<&'static str>) {
    let mut lock = GLOBAL_ROOT.lock().unwrap();
    if lock.is_none() {
        let mut root = match tmp_dir {
            Some(tmp_dir) => PathBuf::from(tmp_dir),
            None => global_root_legacy(),
        };

        root.push(CARGO_INTEGRATION_TEST_DIR);
        *lock = Some(root);
    }
}

pub fn global_root() -> PathBuf {
    let lock = GLOBAL_ROOT.lock().unwrap();
    match lock.as_ref() {
        Some(p) => p.clone(),
        None => unreachable!("GLOBAL_ROOT not set yet"),
    }
}

// We need to give each test a unique id. The test name could serve this
// purpose, but the `test` crate doesn't have a way to obtain the current test
// name.[*] Instead, we used the `cargo-test-macro` crate to automatically
// insert an init function for each test that sets the test name in a thread
// local variable.
//
// [*] It does set the thread name, but only when running concurrently. If not
// running concurrently, all tests are run on the main thread.
thread_local! {
    static TEST_ID: RefCell<Option<usize>> = RefCell::new(None);
}

pub struct TestIdGuard {
    _private: (),
}

pub fn init_root(tmp_dir: Option<&'static str>) -> TestIdGuard {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    TEST_ID.with(|n| *n.borrow_mut() = Some(id));

    let guard = TestIdGuard { _private: () };

    set_global_root(tmp_dir);
    let r = root();
    r.rm_rf();
    r.mkdir_p();

    guard
}

impl Drop for TestIdGuard {
    fn drop(&mut self) {
        TEST_ID.with(|n| *n.borrow_mut() = None);
    }
}

pub fn root() -> PathBuf {
    let id = TEST_ID.with(|n| {
        n.borrow().expect(
            "Tests must use the `#[cargo_test]` attribute in \
             order to be able to use the crate root.",
        )
    });

    let mut root = global_root();
    root.push(&format!("t{}", id));
    root
}

pub fn home() -> PathBuf {
    let mut path = root();
    path.push("home");
    path.mkdir_p();
    path
}

pub trait CargoPathExt {
    fn rm_rf(&self);
    fn mkdir_p(&self);

    fn move_into_the_past(&self) {
        self.move_in_time(|sec, nsec| (sec - 3600, nsec))
    }

    fn move_into_the_future(&self) {
        self.move_in_time(|sec, nsec| (sec + 3600, nsec))
    }

    fn move_in_time<F>(&self, travel_amount: F)
    where
        F: Fn(i64, u32) -> (i64, u32);
}

impl CargoPathExt for Path {
    fn rm_rf(&self) {
        let meta = match self.symlink_metadata() {
            Ok(meta) => meta,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return;
                }
                panic!("failed to remove {:?}, could not read: {:?}", self, e);
            }
        };
        // There is a race condition between fetching the metadata and
        // actually performing the removal, but we don't care all that much
        // for our tests.
        if meta.is_dir() {
            if let Err(e) = remove_dir_all::remove_dir_all(self) {
                panic!("failed to remove {:?}: {:?}", self, e)
            }
        } else if let Err(e) = fs::remove_file(self) {
            panic!("failed to remove {:?}: {:?}", self, e)
        }
    }

    fn mkdir_p(&self) {
        fs::create_dir_all(self)
            .unwrap_or_else(|e| panic!("failed to mkdir_p {}: {}", self.display(), e))
    }

    fn move_in_time<F>(&self, travel_amount: F)
    where
        F: Fn(i64, u32) -> (i64, u32),
    {
        if self.is_file() {
            time_travel(self, &travel_amount);
        } else {
            recurse(self, &self.join("target"), &travel_amount);
        }

        fn recurse<F>(p: &Path, bad: &Path, travel_amount: &F)
        where
            F: Fn(i64, u32) -> (i64, u32),
        {
            if p.is_file() {
                time_travel(p, travel_amount)
            } else if !p.starts_with(bad) {
                for f in t!(fs::read_dir(p)) {
                    let f = t!(f).path();
                    recurse(&f, bad, travel_amount);
                }
            }
        }

        fn time_travel<F>(path: &Path, travel_amount: &F)
        where
            F: Fn(i64, u32) -> (i64, u32),
        {
            let stat = t!(path.symlink_metadata());

            let mtime = FileTime::from_last_modification_time(&stat);

            let (sec, nsec) = travel_amount(mtime.unix_seconds(), mtime.nanoseconds());
            let newtime = FileTime::from_unix_time(sec, nsec);

            // Sadly change_file_times has a failure mode where a readonly file
            // cannot have its times changed on windows.
            do_op(path, "set file times", |path| {
                filetime::set_file_times(path, newtime, newtime)
            });
        }
    }
}

// Replace with std implementation when stabilized, see
// https://github.com/rust-lang/rust/issues/85748
pub fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

fn do_op<F>(path: &Path, desc: &str, mut f: F)
where
    F: FnMut(&Path) -> io::Result<()>,
{
    match f(path) {
        Ok(()) => {}
        Err(ref e) if e.kind() == ErrorKind::PermissionDenied => {
            let mut p = t!(path.metadata()).permissions();
            p.set_readonly(false);
            t!(fs::set_permissions(path, p));

            // Unix also requires the parent to not be readonly for example when
            // removing files
            let parent = path.parent().unwrap();
            let mut p = t!(parent.metadata()).permissions();
            p.set_readonly(false);
            t!(fs::set_permissions(parent, p));

            f(path).unwrap_or_else(|e| {
                panic!("failed to {} {}: {}", desc, path.display(), e);
            })
        }
        Err(e) => {
            panic!("failed to {} {}: {}", desc, path.display(), e);
        }
    }
}

/// Get the filename for a library.
///
/// `kind` should be one of: "lib", "rlib", "staticlib", "dylib", "proc-macro"
///
/// For example, dynamic library named "foo" would return:
/// - macOS: "libfoo.dylib"
/// - Windows: "foo.dll"
/// - Unix: "libfoo.so"
pub fn get_lib_filename(name: &str, kind: &str) -> String {
    let prefix = get_lib_prefix(kind);
    let extension = get_lib_extension(kind);
    format!("{}{}.{}", prefix, name, extension)
}

pub fn get_lib_prefix(kind: &str) -> &str {
    match kind {
        "lib" | "rlib" => "lib",
        "staticlib" | "dylib" | "proc-macro" => {
            if cfg!(windows) {
                ""
            } else {
                "lib"
            }
        }
        _ => unreachable!(),
    }
}

pub fn get_lib_extension(kind: &str) -> &str {
    match kind {
        "lib" | "rlib" => "rlib",
        "staticlib" => {
            if cfg!(windows) {
                "lib"
            } else {
                "a"
            }
        }
        "dylib" | "proc-macro" => {
            if cfg!(windows) {
                "dll"
            } else if cfg!(target_os = "macos") {
                "dylib"
            } else {
                "so"
            }
        }
        _ => unreachable!(),
    }
}

/// Returns the sysroot as queried from rustc.
pub fn sysroot() -> String {
    let output = Command::new("rustc")
        .arg("--print=sysroot")
        .output()
        .expect("rustc to run");
    assert!(output.status.success());
    let sysroot = String::from_utf8(output.stdout).unwrap();
    sysroot.trim().to_string()
}

pub fn echo_wrapper() -> std::path::PathBuf {
    let p = project()
        .at("rustc-echo-wrapper")
        .file("Cargo.toml", &basic_manifest("rustc-echo-wrapper", "1.0.0"))
        .file(
            "src/main.rs",
            r#"
            fn main() {
                let args = std::env::args().collect::<Vec<_>>();
                eprintln!("WRAPPER CALLED: {}", args[1..].join(" "));
                let status = std::process::Command::new(&args[1])
                    .args(&args[2..]).status().unwrap();
                std::process::exit(status.code().unwrap_or(1));
            }
            "#,
        )
        .build();
    p.cargo("build").run();
    p.bin("rustc-echo-wrapper")
}
