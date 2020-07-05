# Static methods possibilities
Adding method directly to the class, breaks the "no new field" rule
```
class Foo {}

Foo.new = () {...}
```
Metaclass monkey patching, requires two different blocks
```
class Foo {}

class Foo.$class {
  new = () {...}
}
```
Bind the self in a class block to the class
```
class Foo {
  self.new = () {...}
}
```
