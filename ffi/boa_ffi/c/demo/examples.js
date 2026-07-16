//Basic addition
2 + 2

//String concatenation
"hello" + " " + "world" + "!"

//Template literals
`2 + 2 = ${2 + 2}`

//Reduce
[1,2,3,4].reduce((total, x) => total += x)

//Objects and json
JSON.stringify({name: "Boa", lang: "JS", version: 1.0})

//Classes and engine persistance
class Point { constructor(x,y){ this.x=x; this.y=y; } dist(){ return Math.sqrt(this.x**2+this.y**2); } }
//then use that defined class afterwards, showing persistance
new Point(3,4).dist()

//Recursion
(function fib(n) { return n < 2 ? n : fib(n-1) + fib(n-2); }) (15)

//Try catch
try { JSON.parse("{bad json") } catch (e) { e.message }