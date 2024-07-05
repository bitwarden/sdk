use jni::{
    signature::JavaType,
    sys::{jint, jsize, JavaVM},
};

pub fn init() {
    static ANDROID_INIT: std::sync::Once = std::sync::Once::new();
    ANDROID_INIT.call_once(|| {
        let jvm = java_vm();
        let env = jvm.attach_current_thread_permanently().unwrap();
        init_verifier(&env).unwrap();
    });
}

type JniGetCreatedJavaVms =
    unsafe extern "system" fn(vmBuf: *mut *mut JavaVM, bufLen: jsize, nVMs: *mut jsize) -> jint;
const JNI_GET_JAVA_VMS_NAME: &[u8] = b"JNI_GetCreatedJavaVMs";

fn java_vm() -> jni::JavaVM {
    // should be jni_onload，but uniffi use jna
    // https://github.com/mozilla/uniffi-rs/issues/1778#issuecomment-1807979746
    let lib = libloading::os::unix::Library::this();
    let get_created_java_vms: JniGetCreatedJavaVms =
        unsafe { *lib.get(JNI_GET_JAVA_VMS_NAME).unwrap() };

    let mut created_java_vms: [*mut JavaVM; 1] = [std::ptr::null_mut() as *mut JavaVM];
    let mut java_vms_count: i32 = 0;
    let ok = unsafe { get_created_java_vms(created_java_vms.as_mut_ptr(), 1, &mut java_vms_count) };
    assert_eq!(ok, jni::sys::JNI_OK);
    assert_eq!(java_vms_count, 1);

    let jvm_ptr = created_java_vms[0];
    let jvm = unsafe { jni::JavaVM::from_raw(jvm_ptr) }.unwrap();
    jvm
}

fn init_verifier(env: &jni::JNIEnv<'_>) -> jni::errors::Result<()> {
    let activity_thread = "android/app/ActivityThread";
    let current_activity_thread = env.get_static_method_id(
        &activity_thread,
        "currentActivityThread",
        "()Landroid/app/ActivityThread;",
    )?;
    let at = env
        .call_static_method_unchecked(
            &activity_thread,
            current_activity_thread,
            JavaType::Object("android/app/ActivityThread".to_string()),
            &[],
        )?
        .l()?;

    let get_application = env.get_method_id(
        activity_thread,
        "getApplication",
        "()Landroid/app/Application;",
    )?;
    let context = env
        .call_method_unchecked(
            at,
            get_application,
            JavaType::Object("android/app/Application".to_string()),
            &[],
        )?
        .l()?;

    Ok(rustls_platform_verifier::android::init_hosted(
        &env, context,
    )?)
}
