pub struct ConcurrentAction {
   // inner callback
   callback: Box<Fn() + Send + 'static>,

} impl ConcurrentAction {

   // set innner callback
   pub fn new<T>(data: T) -> ConcurrentAction where  T: Fn() + Send + 'static  {
      return ConcurrentAction {
         callback: Box::new(data),
      };
   }

   // run inner callback
   pub fn invoke(&self) {
      (self.callback)()
   }
}
