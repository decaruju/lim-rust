# Proxy methods are like python's magic methods
Python uses dunder to prevent collisions (long to write)
Rust uses separate impl blocks (multiple blocks)
Ruby uses the operator as the method name (arcane)
C++ uses the operator keyword (keywords bad)

maybe prefix with $?

```
class Complex {
  real
  imaginary
  
  self.new = (real, imaginary) {
    rtn = Complex()
    rtn.real = real
    rtn.imaginary = imaginary
    rtn
  }
  
  $add = (rhs) {
    Complex.new(
      self.real + rhs.real,
      self.imaginary + rhs.imaginary
    )
  }
  
}

c1 = Complex.new(1, 1)
c2 = Complex.new(-1, 2)
c1.$add(c2) == c1 + c2 # the + is replaced with the $add method
```
