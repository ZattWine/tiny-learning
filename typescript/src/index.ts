/*=============================================
=            Basic types            =
=============================================*/

let id: number = 5
let company: string = 'Yoyo'
let isPublished: boolean = true
let x: any = 'Hello'

let ids: number[] = [1, 2, 3, 4, 5, 6]
let array: any[] = [1, 'two', false]

/*=====  End of Basic types  ======*/

/*=============================================
=            Turple and Turple array          =
=============================================*/

// Turple
let person: [number, string, boolean] = [1, 'two', true]

// Turple array
let employee: [number, string][]
employee = [
  [1, 'one'],
  [2, 'two'],
  [3, 'three'],
]

/*=====  End of Turple and Turple array  ======*/

/*=============================================
=            Union            =
=============================================*/

let uid: string | number = 22
uid = 'twenty two'

/*=====  End of Union  ======*/

/*=============================================
=            Enum            =
=============================================*/

enum Direction1 {
  Up = 1,
  Down,
  Left,
  Right,
}

enum Direction2 {
  Up = 'Up',
  Down = 'Down',
  Left = 'Left',
  Right = 'Right',
}

/*=====  End of Enum  ======*/

/*=============================================
=            Objects            =
=============================================*/

type User = {
  id: number
  name: string
}

const user: User = {
  id: 1,
  name: 'John Doe',
}

/*=====  End of Objects  ======*/

/*=============================================
=            Type assertions            =
=============================================*/

let cid: any = 1
// let customerId = <number>cid
let customerId = cid as number

/*=====  End of Type assertions  ======*/

/*=============================================
=            Functions            =
=============================================*/

function addNum(x: number, y: number): number {
  return x + y
}

function log(message: string | number): void {
  console.log(message)
}

log(addNum(4, 4))

/*=====  End of Functions  ======*/

/*=============================================
=            Interfaces            =
=============================================*/

interface UserInterface {
  readonly id: number
  name: string
  age?: number
}

const myUser: UserInterface = {
  id: 1,
  name: 'John Doe',
}

interface MathFunc {
  (x: number, y: number): number
}

const add: MathFunc = (x: number, y: number): number => x + y
const subtract: MathFunc = (x: number, y: number): number => x - y

/*=====  End of Interfaces  ======*/

/*=============================================
=            Classes            =
=============================================*/

interface PersonInterface {
  readonly id: number
  name: string
  register(): string
}

class Person implements PersonInterface {
  // private, protected, public
  id: number
  name: string

  constructor(id: number, name: string) {
    this.id = id
    this.name = name
  }

  register() {
    return `${this.name} is now registered.`
  }
}

const zatt = new Person(1, 'Zatt')
const norm = new Person(2, 'Norm')

class Employee extends Person {
  position: string

  constructor(id: number, name: string, position: string) {
    super(id, name)
    this.position = position
  }
}

const emp = new Employee(201, 'Shoon', 'Developer')
console.log(emp.register())

/*=====  End of Classes  ======*/

/*=============================================
=            Generics            =
=============================================*/

function getArray<T>(items: T[]): T[] {
  return new Array().concat(items)
}

let numArr = getArray([1, 2, 3, 4, 5])
let strArr = getArray(['one', 'two', 'three'])

/*=====  End of Generics  ======*/
