# Classes can be reopened anywhere to add fields and methods
```
class Foo {
  field1
  
  method1 = () {...}
}

class Foo {
  field2
  
  method2 = () {...}
}

foo = Foo()
foo.field1 = 1
foo.field2 = 1
foo.method1()
foo.method2()
```
