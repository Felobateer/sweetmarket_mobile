import 'package:flutter/material.dart';
import 'package:sweetmarket/services/models/categories.dart';
import 'package:sweetmarket/services/models/filters.dart';
import 'package:sweetmarket/services/models/products.dart';
import 'package:sweetmarket/widgets/boxes/appBar.dart';
import 'package:sweetmarket/widgets/boxes/categoryCard.dart';
import 'package:sweetmarket/widgets/boxes/filterDropdown.dart';
import 'package:sweetmarket/widgets/boxes/productCard.dart';

class Market extends StatelessWidget {
  const Market({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: PreferredSize(
        preferredSize: const Size.fromHeight(56),
        child: Appbar(
          right: Row(
            children: [
              Padding(
                padding: const EdgeInsets.all(5.0),
                child: Image.asset(
                  'assets/imgs/sweetmarketLogo.png',
                  width: 48,
                  height: 48,
                  fit: BoxFit.cover,
                ),
              ),
              Expanded(
                child: Container(
                  padding: const EdgeInsets.only(left: 10, right: 10),
                  child: const SearchInput(),
                ),
              ),
            ],
          ),
        ),
      ),
      body: ListView(
        padding: const EdgeInsets.all(10),
        children: [
          const SizedBox(height: 10),
          SizedBox(
            height: 50,
            child: FilterDropdowns(filters: filters),
          ),
          const SizedBox(height: 10),
          CategoryList(),
          const SizedBox(height: 10),
          ProductList(),
        ],
      ),
    );
  }
}

class SearchInput extends StatelessWidget {
  final String hint;

  const SearchInput({super.key, this.hint = 'Vanilla Ice Cream'});

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10),
      decoration: BoxDecoration(
        color: Colors.white70, // Background color
        borderRadius: BorderRadius.circular(8), // Rounded corners
      ),
      child: Row(
        children: [
          const Icon(
            Icons.search,
            color: Colors.black54, // Magnifying glass color
          ),
          const SizedBox(width: 8), // Spacing between icon and text
          Expanded(
            child: TextField(
              style: const TextStyle(color: Colors.grey), // Text color
              decoration: InputDecoration(
                hintText: hint,
                hintStyle:
                    const TextStyle(color: Colors.grey), // Hint text color
                border: InputBorder.none, // Remove underline
                contentPadding: const EdgeInsets.only(bottom: 4),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class FilterDropdowns extends StatelessWidget {
  final List<Filter> filters;

  const FilterDropdowns({required this.filters, super.key});

  @override
  Widget build(BuildContext context) {
    // Ensure filters is not null by using an empty list as fallback.
    final safeFilters = filters ?? [];

    return ListView.builder(
      scrollDirection: Axis.horizontal,
      itemCount: safeFilters.length,
      itemBuilder: (context, index) {
        return Padding(
          padding: const EdgeInsets.only(right: 10),
          child: Filterdropdown(filter: safeFilters[index]),
        );
      },
    );
  }
}

class CategoryList extends StatelessWidget {
  const CategoryList({super.key});

  @override
  Widget build(BuildContext context) {
    // Ensure categories is not null by using an empty list as fallback.
    final safeCategories = categories ?? [];

    return Padding(
      padding: EdgeInsets.symmetric(horizontal: 20),
      child: SizedBox(
        height: 175,
        child: ListView.builder(
          scrollDirection: Axis.horizontal,
          itemCount: safeCategories.length,
          itemBuilder: (context, index) {
            return Padding(
              padding: const EdgeInsets.only(right: 20),
              child: CategoryCard(category: safeCategories[index]),
            );
          },
        ),
      ),
    );
  }
}

class ProductList extends StatelessWidget {
  const ProductList({super.key});

  @override
  Widget build(BuildContext context) {
    // Ensure products is not null by using an empty list as fallback.
    final safeProducts = products ?? [];

    return Padding(
      padding: EdgeInsets.symmetric(horizontal: 20),
      child: ListView.builder(
        scrollDirection: Axis.vertical,
        itemCount: safeProducts.length,
        itemBuilder: (context, index) {
          return Padding(
            padding: const EdgeInsets.only(right: 20),
            child: Productcard(product: safeProducts[index]),
          );
        },
      ),
    );
  }
}
