use std::os::raw::c_char;
use std::ffi::{CStr, CString};
use std::io::{BufRead, Write};

fn ptr_to_string(str: *const c_char) -> String {
    let c_buf: *const c_char = str;
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    str_slice.to_owned()
}

#[no_mangle]
pub extern "C" fn library_signatures() -> *const c_char {
    let c_string = CString::new("
    const char* println(const char* ptr); 
    const char* print(const char* ptr);
    const char* input(const char* ptr);
    const char* read_file(const char* filelocation);
    const char* write_file(const char* filelocation, const char* content);
    ").expect("CString::new failed");
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn println(ptr: *const c_char) {
    let str = ptr_to_string(ptr);
    println!("{}", str);
}

#[no_mangle]
pub extern "C" fn print(ptr: *const c_char) {
    let str = ptr_to_string(ptr);
    print!("{}", str);
}

#[no_mangle]
pub extern "C" fn input(ptr: *const c_char) -> *const c_char {
    // print(ptr), has a really weird but where it 
    // does not print your message before reading the std, but after...
    println(ptr);

    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line).unwrap();
    CString::new(line).expect("Could not parse input line").into_raw()
}

#[no_mangle]
pub extern "C" fn read_file(filelocation: *const c_char) -> *const c_char {
    let res = std::fs::read_to_string(ptr_to_string(filelocation)).expect("An error occured while reading the file");
    CString::new(res).expect("Could not parse content of file").into_raw()
}

#[no_mangle]
pub extern "C" fn write_file(filelocation: *const c_char, content: *const c_char) -> bool {
    let file = std::fs::File::create(ptr_to_string(filelocation));
    if let Err(_) = file {
        return false;
    }
    match file {
        Err(_) => return false,
        Ok(mut ok) => {
            let res = ok.write_all(ptr_to_string(content).as_bytes());
            if let Err(_) = res {
                return false;
            }
        }
    }

    true
}