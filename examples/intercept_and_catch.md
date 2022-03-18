# Intercept catches and reraises

try {
  failure()
} intercept(error) {
  log(error)
}

is equivalent to

try {
  failure()
  } catch(error) {
    log(error)
    throw error
  }
