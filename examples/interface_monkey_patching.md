# Methods can be patched on all classes implementing a method

```
class Foo {
  method1 = () {}
}
class Bar {
  field1

  method1 = () {}
  method2 = () {}
}
class Baz {
}

class [method1] {
  double_method_1 = () {
    self.method1()
    self.method1()
  }
}

foo = Foo()
foo.double_method_1() # patched on
bar = Bar()
bar.double_method_1() # patched on
baz = Baz()
baz.double_method_1() # panic
```

no check is made on the fields and methods used in patched methods

```
class Foo {
  method1 = () {}
}
class Bar {
  field1

  method1 = () {}
  method2 = () {}
}

class [method1] {
  method_1_2 = () {
    self.method1()
    self.method2() # fine
  }
  
  get_field_1 = () {
    self.field1 # fine
  }
}

bar = Bar()
bar.double_method_1() # works
foo.get_field_1() # panic in method when accessing field1

foo = Foo()
foo.double_method_1() # panic in method when calling method2
foo.get_field_1() # panic in method when accessing field1
```

The typical use case is a printer

```
class Foo {
  $string = () {
    'My Foo object'
  }
}

class [$string] {
  print = () {
    print(self.$string())
  }
}

foo = Foo()
foo.print()
```
