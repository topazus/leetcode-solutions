let rec f = function
  | a :: (b :: c as t) -> if a = b then f t else a :: f t
  | x -> x

let rec f2 nums =
  match nums with
  | a :: (b :: _ as t) -> if a = b then f2 t else a :: f2 t
  | x -> x

let rec f3 nums =
  match nums with
  | h :: t -> if t != [] && h = List.hd t then f3 t else h :: f3 t
  | x -> x
