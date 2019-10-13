import proto_example
from proto_example.generated.function_a.v1.function_pb2 import Request

def main():
    req = Request()
    req.descriptions = "hello from python"

    res = proto_example.function_a(req)

    print("rust response:", res)


if __name__ == '__main__':
    main()