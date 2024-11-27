import 'package:flutter/material.dart';
import 'package:sweetmarket/services/models/categories.dart';
import 'package:sweetmarket/services/models/filters.dart';
import 'package:sweetmarket/services/models/products.dart';
import 'package:sweetmarket/widgets/boxes/appBar.dart';
import 'package:sweetmarket/widgets/boxes/categoryCard.dart';
import 'package:sweetmarket/widgets/boxes/filterDropdown.dart';
import 'package:sweetmarket/widgets/boxes/marketCard.dart';
import 'package:sweetmarket/widgets/boxes/productCard.dart';

class Market extends StatelessWidget {
  const Market({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: MarketAppbar(),
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

class MarketAppbar extends StatelessWidget implements PreferredSizeWidget {
  const MarketAppbar({super.key});

  @override
  Widget build(BuildContext context) {
    return Appbar(
      left: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Image.asset(
          'assets/imgs/sweetmarketLogo.png',
          fit: BoxFit.cover,
        ),
      ),
      right: Expanded(
        child: Container(
          padding:
              const EdgeInsets.only(right: 10, left: 56, top: 8, bottom: 8),
          child: const SearchInput(),
        ),
      ),
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(kToolbarHeight);
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
    return ListView.builder(
      scrollDirection: Axis.horizontal,
      itemCount: filters.length,
      itemBuilder: (context, index) {
        return Padding(
          padding: const EdgeInsets.only(right: 10),
          child: Filterdropdown(filter: filters[index]),
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

    return Padding(
      padding: EdgeInsets.symmetric(horizontal: 20),
      child: SizedBox(
        height: 175,
        child: ListView.builder(
          scrollDirection: Axis.horizontal,
          itemCount: categories.length,
          itemBuilder: (context, index) {
            return Padding(
              padding: const EdgeInsets.only(right: 20),
              child: CategoryCard(category: categories[index]),
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
    return Center(
      child: Wrap(
        spacing: 5.0,
        runSpacing: 4.0,
        children: products.map((product) {
          return Marketcard(product: product);
        }).toList(),
      ),
    );
  }
}
