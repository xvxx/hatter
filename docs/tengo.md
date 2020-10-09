<!--
    Tengo Language
      convert to
    Hatter Language
-->

# variable definition and primitive types
a := "foo"   # String
b := -19.84  # Number
c := 5       # Number
d := true    # Bool
e := '九'    # String
print("a: ", a)
print("b: ", b)
print("c: ", c)
print("d: ", d)
print("e: ", e)

# assignment
b = "bar"    # error: can't assign value of different type

# map and list
m := {a: {b: {c: [1, 2, 3]}}}
print("m: ", m)

# indexing with .
bb := { first: "Bilbo", last: "Baggins" }
name := "{bb.first} {bb.last}"
print("name: ", name)

# slicing TODO
str := "hello world"
print(str[1:5])    # "ello"
arr := [1, 2, 3, 4, 5]
print(arr[2:4])    # [3, 4]

# functions
each := fn(seq, f)
    # array iteration
    for x in seq
        f(x)

sum := fn(seq)
   s := 0
   each(seq, fn(x)
       s += x    # closure: capturing variable 's'
   return s
print("sum: ", sum([1, 2, 3])) # 6

map-to-array := fn(m)
    arr := []
    # map iteration
    for key, value in m
        arr = append(arr, key, value)  # builtin function 'append'
    return arr

m-arr := map-to-array(m)
print(m-arr, " (len: ", len(m-arr), ")")

# tail-call optimization: faster and enables loop via recursion
count-odds := fn(n, c)
	if n == 0
		return c
	else if n % 2 == 1
	    c++
	return count-odds(n-1, c)

num-odds := count-odds(100000, 0)
print(num-odds) # 50000

# type coercion
s1 := string(1984)    # "1984"
i2 := int("-999")     # -999
f3 := float(-51)      # -51.0
b4 := bool(1)         # true
c5 := char(88)        # 'X'

# if statement
three := 3
if three > 2
    print("three > 2")
else if three == 2
    print("three = 2")
else
    print("three < 2")

# for statement
seven := 0
arr2 := [1, 2, 3, 1]
for v in arr2
    seven += arr2[i]
print("seven: ", seven)
