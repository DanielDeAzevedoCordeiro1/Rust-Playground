import ctypes

lib = ctypes.CDLL("./libmathfix.so")

lib.add_u64.argtypes = [
    ctypes.c_uint64,
    ctypes.c_uint64,
    ctypes.POINTER(ctypes.c_uint64),
]

lib.multiply_u64.argtypes = [
    ctypes.c_uint64,
    ctypes.c_uint64,
    ctypes.POINTER(ctypes.c_uint64),
]

lib.subtract_u64.argtypes = [
    ctypes.c_uint64,
    ctypes.c_uint64,
    ctypes.POINTER(ctypes.c_uint64),
]

lib.divide_u64.argtypes = [
    ctypes.c_uint64,
    ctypes.c_uint64,
    ctypes.POINTER(ctypes.c_uint64),
]


lib.add_u64.restype = ctypes.c_int
lib.multiply_u64.restype = ctypes.c_int
lib.subtract_u64.restype = ctypes.c_int
lib.divide_u64.restype = ctypes.c_int

result_add = ctypes.c_uint64()
result_mult = ctypes.c_uint64()
result_sub = ctypes.c_uint64()
result_div = ctypes.c_uint64()

status_add = lib.add_u64(100, 100, ctypes.byref(result_add))
status_mult = lib.multiply_u64(10, 1000, ctypes.byref(result_mult))
status_sub = lib.subtract_u64(200, 50, ctypes.byref(result_sub))
status_div = lib.divide_u64(1000, 10, ctypes.byref(result_div))

if status_add == 0:
    print("Resultado =", result_add.value)
elif status_add == 1:
    print("Erro: Overflow")
else:
    print("Erro desconhecido:", status_add)


if status_mult == 0:
    print("Resultado =", result_mult.value)
elif status_mult == 1:
    print("Erro: Overflow")
else:
    print("Erro desconhecido:", status_mult)
