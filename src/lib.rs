#[macro_export]
macro_rules! init {
  () => {
#[allow(clippy::all)]
mod exports {
    #[derive(Clone)]
    pub struct ModuleResponse {
        pub status:u16,pub body:Vec<u8>,pub headers:Vec<ResponseHeader>,
    }
    impl core::fmt::Debug for ModuleResponse {
        fn fmt(&self,f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ModuleResponse").field("status", &self.status).field("body", &self.body).field("headers", &self.headers).finish()
        }
    
        }
    #[derive(Clone)]
    pub struct ResponseHeader {
        pub name:Vec<u8>,pub value:Vec<u8>,
    }
    impl core::fmt::Debug for ResponseHeader {
        fn fmt(&self,f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ResponseHeader").field("name", &self.name).field("value", &self.value).finish()
        }
    
        }
    #[export_name = "handler"]
    unsafe extern "C" fn __wai_bindgen_exports_handler(arg0:i32,arg1:i32,) -> i32 {
        let len0 = arg1 as usize;
        let result =  <super::Exports as Exports>::handler(Vec::from_raw_parts(arg0 as *mut _,len0,len0));
        let ptr1 = EXPORTS_RET_AREA.0.as_mut_ptr()as i32;
        let ModuleResponse {
            status:status2,body:body2,headers:headers2,
        } = result;
        *((ptr1+0)as *mut u16) = (status2 as i32) as u16;
        let vec3 = (body2).into_boxed_slice();
        let ptr3 = vec3.as_ptr()as i32;
        let len3 = vec3.len()as i32;
        core::mem::forget(vec3);
        *((ptr1+8)as *mut i32) = len3;
        *((ptr1+4)as *mut i32) = ptr3;
        let vec7 = headers2;
        let len7 = vec7.len()as i32;
        let layout7 = core::alloc::Layout::from_size_align_unchecked(vec7.len()*16,4);
        let result7 = std::alloc::alloc(layout7);
        if result7.is_null(){
            std::alloc::handle_alloc_error(layout7);
        }for(i,e)in vec7.into_iter().enumerate(){
            let base = result7 as i32+(i as i32)*16;
            {
                let ResponseHeader {
                    name:name4,value:value4,
                } = e;
                let vec5 = (name4).into_boxed_slice();
                let ptr5 = vec5.as_ptr()as i32;
                let len5 = vec5.len()as i32;
                core::mem::forget(vec5);
                *((base+4)as *mut i32) = len5;
                *((base+0)as *mut i32) = ptr5;
                let vec6 = (value4).into_boxed_slice();
                let ptr6 = vec6.as_ptr()as i32;
                let len6 = vec6.len()as i32;
                core::mem::forget(vec6);
                *((base+12)as *mut i32) = len6;
                *((base+8)as *mut i32) = ptr6;
            }
        }*((ptr1+16)as *mut i32) = len7;
        *((ptr1+12)as *mut i32) = result7 as i32;
        ptr1
    }
    #[repr(align(4))]
    struct RetArea([u8;
    20]);
    
    static mut EXPORTS_RET_AREA:RetArea = RetArea([0;
    20]);
    pub trait Exports {
        fn handler(params:Vec<u8>,) -> ModuleResponse;
    
        }
}
#[allow(clippy::all)]
mod imports {
    #[derive(Clone)]
    pub struct CalledModuleResponse {
        pub status:u16,pub body:Vec<u8>,pub headers:Vec<ResponseHeader>,
    }
    impl core::fmt::Debug for CalledModuleResponse {
        fn fmt(&self,f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("CalledModuleResponse").field("status", &self.status).field("body", &self.body).field("headers", &self.headers).finish()
        }
    
        }
    #[derive(Clone)]
    pub struct ResponseHeader {
        pub name:Vec<u8>,pub value:Vec<u8>,
    }
    impl core::fmt::Debug for ResponseHeader {
        fn fmt(&self,f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.debug_struct("ResponseHeader").field("name", &self.name).field("value", &self.value).finish()
        }
    
        }
    pub fn call_module(module_name: &[u8],params: &[u8],) -> CalledModuleResponse {
        unsafe {
            let vec0 = module_name;
            let ptr0 = vec0.as_ptr()as i32;
            let len0 = vec0.len()as i32;
            let vec1 = params;
            let ptr1 = vec1.as_ptr()as i32;
            let len1 = vec1.len()as i32;
            let ptr2 = IMPORTS_RET_AREA.0.as_mut_ptr()as i32;
            #[link(wasm_import_module = "imports")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32",link_name = "call-module")]
                #[cfg_attr(not(target_arch = "wasm32"),link_name = "imports_call-module")]
                fn wai_import(_:i32,_:i32,_:i32,_:i32,_:i32,);
            
                }wai_import(ptr0,len0,ptr1,len1,ptr2);
            let len3 =  *((ptr2+8)as *const i32)as usize;
            let base6 =  *((ptr2+12)as *const i32);
            let len6 =  *((ptr2+16)as *const i32);
            let mut result6 = Vec::with_capacity(len6 as usize);
            for i in 0..len6 {
                let base = base6+i*16;
                result6.push({
                    let len4 =  *((base+4)as *const i32)as usize;
                    let len5 =  *((base+12)as *const i32)as usize;
                    ResponseHeader {
                        name:Vec::from_raw_parts(*((base+0)as *const i32)as *mut _,len4,len4),value:Vec::from_raw_parts(*((base+8)as *const i32)as *mut _,len5,len5),
                    }
                });
            }std::alloc::dealloc(base6 as *mut _,std::alloc::Layout::from_size_align_unchecked((len6 as usize)*16,4,),);
            CalledModuleResponse {
                status:i32::from(*((ptr2+0)as *const u16))as u16,body:Vec::from_raw_parts(*((ptr2+4)as *const i32)as *mut _,len3,len3),headers:result6,
            }
        }
    }
    #[repr(align(4))]
    struct RetArea([u8;
    20]);
    
    static mut IMPORTS_RET_AREA:RetArea = RetArea([0;
    20]);
}

struct Exports;
impl exports::Exports for Exports {
    fn handler(params: Vec<u8>) -> ModuleResponse {
        block_on(handler(params))
    }
}
use exports::*;
use futures::executor::block_on;

pub struct ModuleResponseBuilder {
    pub status: Option<u16>,
    pub body: Option<Vec<u8>>,
    pub headers: Option<Vec<ResponseHeader>>,
}

impl ModuleResponseBuilder {
    pub fn new() -> ModuleResponseBuilder {
        ModuleResponseBuilder {
            status: Some(200),
            body: Some(Vec::new()),
            headers: Some(vec![]),
        }
    }

    pub fn status(&mut self, status: u16) -> &mut ModuleResponseBuilder {
        self.status = Some(status);
        self
    }

    pub fn body(&mut self, body: Vec<u8>) -> &mut ModuleResponseBuilder {
        self.body = Some(body);
        self
    }

    pub fn header(&mut self, name: Vec<u8>, value: Vec<u8>) -> &mut ModuleResponseBuilder {
        let mut headers = self.headers.clone().expect("response headers must not be null");
        headers.push(ResponseHeader { name, value });
        self.headers = Some(headers);
        self
    }

    pub fn finish(&self) -> ModuleResponse {
        ModuleResponse {
            status: self.status.expect("response status code must not be null"),
            body: self.body.clone().expect("response body must not be null"),
            headers: self.headers.clone().expect("response headers must not be null"),
        }
    }
}
    }
}
