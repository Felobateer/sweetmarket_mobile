class Category {
  final String name;
  final String photo;

  Category({
    required this.name,
    required this.photo,
  });

  factory Category.fromJson(Map<String, dynamic> json) {
    return Category(
      name: json['name'],
      photo: json['photo'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'name': name,
      'photo': photo,
    };
  }
}

List<Category> categories = [
  Category(name: 'Ice Cream', photo: 'assets/imgs/ice-cream.png'),
  Category(name: 'Chocolate', photo: 'assets/imgs/choco.png'),
  Category(name: 'Cakes', photo: 'assets/imgs/cake.png'),
];
