class A:
    pass


class B(A):
    pass


a = A()
b = B()

print(type(A))
assert type(A) == A.__class__
print(a)
print(A.__bases__)

print(">>------<<")

o = object()
print(type(o))
print(o)
print(object.__bases__)