use super::capi;
use super::leptonica;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[derive(Debug, PartialEq)]
pub struct TessApi {
    pub raw: *mut capi::TessBaseAPI,
    pub data_path_cptr: *mut c_char,
}

impl Drop for TessApi {
    fn drop(&mut self) {
        if !self.data_path_cptr.is_null() {
            // free data_path_cptr
            unsafe {
                CString::from_raw(self.data_path_cptr);
            }
        }
    }
}

impl TessApi {
    pub fn new<'a>(data_path: Option<&'a str>, lang: &'a str) -> Option<TessApi> {
        let data_path_cptr;
        let data_path_cstr;
        match data_path {
            Some(dstr) => {
                data_path_cstr = CString::new(dstr).unwrap();
                data_path_cptr = data_path_cstr.into_raw();
            }
            None => {
                data_path_cptr = ptr::null_mut();
            }
        }

        let mut api = TessApi {
            raw: unsafe { capi::TessBaseAPICreate() },
            data_path_cptr: data_path_cptr,
        };

        unsafe {
            let re = capi::TessBaseAPIInit3(
                api.raw,
                api.data_path_cptr,
                CString::new(lang).unwrap().as_ptr(),
            );

            if re == 0 {
                return Some(api);
            } else {
                api.destroy();
                None
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            capi::TessBaseAPIEnd(self.raw);
            capi::TessBaseAPIDelete(self.raw);
        }
    }

    pub fn set_image(&mut self, img: &leptonica::Pix) {
        unsafe { capi::TessBaseAPISetImage2(self.raw, img.raw as *mut capi::Pix) }
    }

    pub fn recognize(&self) -> i32 {
        unsafe { capi::TessBaseAPIRecognize(self.raw, ptr::null_mut()) }
    }

    pub fn set_rectangle(&mut self, b: &leptonica::Box) {
        let v = b.get_val();
        unsafe {
            capi::TessBaseAPISetRectangle(self.raw, v.x, v.y, v.w, v.h);
        }
    }

    pub fn get_utf8_text(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let re: Result<String, std::str::Utf8Error>;
            let sptr = capi::TessBaseAPIGetUTF8Text(self.raw);
            match CStr::from_ptr(sptr).to_str() {
                Ok(s) => {
                    re = Ok(s.to_string());
                }
                Err(e) => {
                    re = Err(e);
                }
            }
            capi::TessDeleteText(sptr);
            return re;
        }
    }

    pub fn mean_text_conf(&self) -> i32 {
        unsafe { capi::TessBaseAPIMeanTextConf(self.raw) }
    }

    pub fn get_regions(&self) -> Option<leptonica::Boxa> {
        unsafe {
            let boxes = capi::TessBaseAPIGetRegions(self.raw, ptr::null_mut());
            if boxes.is_null() {
                return None;
            }
            return Some(leptonica::Boxa { raw: boxes });
        }
    }

    pub fn get_component_images(
        &self,
        level: capi::TessPageIteratorLevel,
        text_only: bool,
    ) -> Option<leptonica::Boxa> {
        let mut text_only_val: i32 = 0;
        if text_only {
            text_only_val = 1;
        }

        unsafe {
            let boxes = capi::TessBaseAPIGetComponentImages(
                self.raw,
                level,
                text_only_val,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if boxes.is_null() {
                return None;
            }
            return Some(leptonica::Boxa { raw: boxes });
        }
    }
}
