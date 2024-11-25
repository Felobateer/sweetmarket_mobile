import 'package:flutter/material.dart';
import 'package:sweetmarket/services/models/categories.dart'; // Make sure the Category model is imported correctly
import 'package:sweetmarket/widgets/texts/text.dart';

class CategoryCard extends StatelessWidget {
  final Category category;

  const CategoryCard({
    required this.category,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(30),
      ),
      child: Padding(
        padding: const EdgeInsets.all(20.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            ClipRRect(
              borderRadius: BorderRadius.circular(20),
              child: Image.asset(
                category.photo,
                width: 100,
                height: 100,
                fit: BoxFit.fitHeight,
              ),
            ),
            const SizedBox(height: 10),
            customText(
              text: category.name,
            ),
          ],
        ),
      ),
    );
  }
}
