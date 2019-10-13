try:
    from proto_example import core
except ImportError:
    core = None

import proto_example.generated.function_a.v1.function_pb2
import proto_example.generated.function_b.v1.function_pb2

def function_a(req):
    """wrapper"""
    res_bytes = core.function_a(req.SerializeToString())

    res = proto_example.generated.function_a.v1.function_pb2.Response()
    res.ParseFromString(res_bytes)
    return res

