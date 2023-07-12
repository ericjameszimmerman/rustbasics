// src/cxxqt_object.rs

#[cxx_qt::bridge]
mod my_object {

    #[cxx_qt::qobject(qml_uri = "hellorustqtcargo", qml_version = "1.0")]
    #[derive(Default)]
    pub struct Hello {}

    impl qobject::Hello {
        #[qinvokable]
        pub fn say_hello(&self) {
            println!("Hello world!")
        }
    }
}
