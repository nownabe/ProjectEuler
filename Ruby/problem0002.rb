class Fib1
  @@mem = {}
  def self.fib(n)
    @@mem[n] ||= (n <= 1 ? n : fib(n-2) + fib(n-1))
  end
end

sum = 0
n = 0
while (f = Fib1.fib(n)) <= 4_000_000
  sum += f if f.even?
  n += 1
end
puts sum
