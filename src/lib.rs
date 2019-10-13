use pyo3::prelude::*;
use pyo3::types::PyBytes;

mod generated;

use generated::function_a;
use protobuf::Message;
use protobuf;

fn function_a_logic(req: &function_a::v1::function::Request) -> function_a::v1::function::Response {
    dbg!(&req);

    let mut res = function_a::v1::function::Response::new();
    res.set_descriptions("hello from rust".to_owned());
    res
}

#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "function_a")]
    fn function_a(py: Python, req: &PyBytes) -> PyObject {
        let req: function_a::v1::function::Request = protobuf::parse_from_bytes(req.as_bytes()).unwrap();

        let res = function_a_logic(&req);

        let res = PyBytes::new(py, res.write_to_bytes().unwrap().as_slice());
        res.to_object(py)
    }

    Ok(())
}
