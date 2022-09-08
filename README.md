## Genericos

Recordemos que los genericos nos permiten definir tipos, funciones  y metodos parametrizados,
lo que significa que es una abstraccion de un comportamiento y este aplica a varios tipos,
en lugar replicar la logica con diferentes tipos, podemos hacer ese tipo generico.

### Costo de usar genericos

el uso de genericos no genera carga en el performance ya que Rust usa monomorpization, lo que implica
que los genericos son un caracteristica que se procesa en tiempo de compilacion y no en tiempo
de ejecucion.

cuando se crea un generico, el compilador va a buscar donde se instancian y con que tipos y va a generar
el codigo necesario (structs, enums, funciones o metodos) con los tipos concretos.

el dinamismo de los genericos funciona durante la compilacion para generar codigo que es repetitivo
garantizando type safety y no es un dinamismo durante la ejecucion, ya que para ese momento el
codigo generico ya se convirtio en codigo con tipos concretos.

## Traits

un trait define funcionalidad que tiene un tipo particular y puede compartir con otros tipos,
un concepto similar en otros lenguajes son las interfaces.