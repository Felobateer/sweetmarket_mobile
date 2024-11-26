class Filter {
  final String name;
  final List<String> filters;

  Filter({required this.name, required this.filters});
}

List<Filter> filters = [
  Filter(name: 'Type', filters: ['Ice', 'Snacks', 'chocolate', 'cakes']),
  Filter(name: 'Ratings', filters: ['1.0', '2.0', '3.0', '4.0', '5.0']),
  Filter(name: 'Size', filters: ['single', 'family']),
  Filter(name: 'Store', filters: ['Kaufland', 'Bistro Co']),
  Filter(name: 'price', filters: ['0 - 2', '2 - 5', '10 or more']),
];
