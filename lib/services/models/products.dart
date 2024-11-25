class Product {
  final String name;
  final String photo;
  final String store;
  final double price;
  final double rating;
  final double ratersNum;
  final double? discount;

  Product(
      {required this.name,
      required this.photo,
      required this.store,
      required this.price,
      required this.rating,
      required this.ratersNum,
      this.discount});
}

List<Product> products = [
  Product(
      name: 'Chocolate Chip Cookies',
      photo: 'assets/imgs/cookie.jpg',
      store: 'Kaufland',
      price: 2.99,
      rating: 4.8,
      ratersNum: 402,
      discount: 1.99),
  Product(
      name: 'Chocolate cake',
      photo: 'assets/imgs/2in1.jpg',
      store: 'Bistro Co.',
      price: 1.99,
      rating: 4.1,
      ratersNum: 84),
  Product(
    name: 'Macadamia Chip Cookies',
    photo: 'assets/imgs/cookie.jpg',
    store: 'Kaufland',
    price: 2.99,
    rating: 4.9,
    ratersNum: 1302,
  ),
  Product(
      name: 'Fruit cake',
      photo: 'assets/imgs/2in1.jpg',
      store: 'Bistro Co.',
      price: 1.99,
      rating: 4.1,
      ratersNum: 84),
  Product(
      name: 'Dark Chocolate',
      photo: 'assets/imgs/cocobar.jpg',
      store: 'Rewe',
      price: 1.49,
      rating: 4.5,
      ratersNum: 3089),
  Product(
      name: 'Milk Chocolate',
      photo: 'assets/imgs/cocobar.jpg',
      store: 'Rewe',
      price: 1.49,
      rating: 4.5,
      ratersNum: 5089),
  Product(
      name: 'Hazelnut Chocolate',
      photo: 'assets/imgs/cocobar.jpg',
      store: 'Rewe',
      price: 1.49,
      rating: 4.5,
      ratersNum: 3890,
      discount: 0.99),
  Product(
      name: 'Couverture Chocolate',
      photo: 'assets/imgs/cocobar.jpg',
      store: 'Rewe',
      price: 1.49,
      rating: 4.5,
      ratersNum: 1089,
      discount: 0.99),
];
