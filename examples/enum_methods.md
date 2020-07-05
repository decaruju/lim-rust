# Enum methods
Instances of enum can have methods

```
enum Foo {
  Variant1
  Variant2
}

class Foo {
  is_variant1 = () {...}
  to_string = () {...}
}

foo1 = Foo.Variant1()
print(foo1.to_string())
```

Methods can be specific to a single variant

```
class Foo.Variant1 {
  perform_variant1_actions = () {...}
}

Foo.Variant1().perform_variant1_actions()
Foo.Variant2().perform_variant1_actions() # panic
```

Fields can also be added to an enum variant

```
class Foo.Variant1 {
  field1
}

foo1 = Foo.Variant1()
foo1.field1 = 4
```

