from py_test_bindings import *

d_own = D(a=23, b = C_B(32), c=A(), d= [43, 34, 23], e=None, f= [34, 23, 12], g="feerfef", h = range(3, 23), i= {"df": 434}, j={3: 23}, k=[23, 23, 12, 12, 1, 1, 1, 9, 1, 10], m=(32, "def", [32, 232]), n = True)

ser = open("serialized.bytes", "rb").read()
d = deserialize(D, ser)
print(d)

ser_own = serialize(d_own)
print(ser_own)

d_des = deserialize(D, ser_own)
print(d_des)