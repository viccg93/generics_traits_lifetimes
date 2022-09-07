
//funcion que permite encontrar el integer mas grande en un slice de u32
//permite usar esta logica con diferentes slices sin repetir el codigo
//aunque tambien esta limitada al tipo u32
pub fn get_largest_integer (list: &[u32]) -> &u32 {
    let mut largest = &list[0]; // obtenemos una referencia el primer elemento

    //comparamos cada refencia y vamos aignando la mayor en largest
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    &largest
}

//si queremos hacer lo mismo con elementos de un slice char
//esta funcion es identica a get_largest_integer, solo cambian los tipos que admite
//este es un buen escenario donde usar una funcion generica
pub fn get_largest_char (list: &[char]) -> &char {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    &largest
}

//funcion generica, donde el tipo esta parmatrizado
//se indica que el parametro <T> en la funcion y podemos usarlo en los params,
//tipo de retorno e incluso en el cuerpo de la funcion
//como el tipo T puede ser cualquier tipo, la operacion binaria de comparacion no esta
//garantizada, para garantizarla se debe delimitar el tipo a generico a tipos que implementen
//esa caracterista, en este caso se delimita a los tipos que implementan el trait std::cmp::PartialOrd
pub fn get_largest<T: std::cmp::PartialOrd> (list: &[T]) -> &T {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    &largest
}

//structs genericos
//al igual que en las funciones no esta limitada la cantidad de parametros genericos
//pero cada parametro queda amarrado al tipo que se asigna al instanciar, si se indica que es u32
//y se asigan un valor de tipo distinto, eso causar un error de compilacion.
pub struct Point<T,U>{
    x: T,
    y: U,
}

//enum genericos
//la definicion de Option es generica, es decir puede funcionar con distintos tipos
enum Option<T> {
    Some(T),
    None
}

//en el caso de Result se usan 2 parametros, el tipo de retorno en ok y el tipo de error
enum Result<T,E> {
    Ok(T),
    Err(E),
}

//metodos genericos
// usaremos el siguiente struct generico
struct Point3d<T> {
    x:T,
    y:T,
    z:T,
}

//los paremetros genericos se indican nuevamente en el bloque impl
//y ya no es necesario indicar estos parametros en los bloques fn
//ya que estamos indicando que esos metodos aplican a Point3d<T>
//osea a cualquier tipo concreto que tome T, puede invocar los metodos en ese bloque
impl <T> Point3d <T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//y tambien podemos delimitar ciertos metodos a tipos concretos
//en este caso, los metodos declarados en el bloque impl solo
//aplican a las instancias de tipo f32

impl Point3d <f32> {
    fn distance_from_origin(&self) -> f32 {
        //teorema de pitagoras
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//Cuando se usa un metodo generico Rust detecta que el parametro que se esta indicando tiene
//relacion con el que se declara en el struct, y por convencion se usa el mismo, aunque no es obligatorio
//un ejemplo de esto son los bloques struct Point3d<T> y impl <T> Point3d <T>, donde se lee que los metodos
//de ese bloque impl van a aplicar a Point3d con cualquier <T>

//las definiciones de los tipos genericos en la signature de los structs, no siempre son los mismo que los de
//la signature de los metodos, cuando es la misma no se indica en el bloque fn, por que esta de facto el tipo de self
//pero puede que se use una instancia distinta de self, y este un caso donde se requiere indicar estos parametros genericos
//en el bloque fn, como en el siguiente ejemplo:

impl <X1,Y1> Point <X1,Y1> {
    //como vamos a usar una instancia de Point con tipos que pueden ser distintos de los de la instancia que llama a este metodo
    //tenemos que declarar estos genericos en la funcion, estos van despues del nombre de la funcion
    fn mixup <X2,Y2> (self, other: Point<X2,Y2>) -> Point<X1,Y2> {
        //como se reciben valores, se hace borrow y ya no se pueden usan ninguna de las instancias originales
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
pub fn generics_code(){
    let vec = vec![1,2,3,4,5];
    let result = get_largest(&vec);

    //como en la definicion de Point, los tipos de los campos son genericos independientes
    //estos pueden ser de tipos distintos
    let p1 = Point {
        x:1,
        y:2.0
    };

    let p2 = Point {
        x:1,
        y:2
    };

    let point_3d = Point3d {
        x:2.0,
        y: 3.0,
        z: 1.0,
    };
    //como la instancias es float, se puede llamar el metodo sin problema
    let distance_xy_axis = point_3d.distance_from_origin();
    println!("la distancia desde el origen en el plano xy es: {}", distance_xy_axis);

    //cuando la instancia no es de tipo float, no se puede llamar al metodo distance_from_origin()
    let point_3d = Point3d {
        x:1,
        y: 2,
        z: 3,
    };
    //let distance_xy_axis = point_3d.distance_from_origin(); //error

    let p1 = Point {x:1,y:2};
    let p2 = Point {x:'a',y:'b'};

    let mixed_point = p1.mixup(p2);

    println!("el valor de p1 es x:{}, y:{}", mixed_point.x,mixed_point.y);

    //p1 y p2 en este punto ya no son owners
    //println!("el valor de p1 es x:{}, y:{}", p1.x,p2.y);

}