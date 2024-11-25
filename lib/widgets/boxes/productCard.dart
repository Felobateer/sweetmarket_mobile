import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:sweetmarket/services/models/products.dart';
import 'package:sweetmarket/widgets/texts/subText.dart';
import 'package:sweetmarket/widgets/texts/text.dart';

class Productcard extends StatelessWidget {
  final Product product;
  const Productcard({required this.product, super.key});

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
            Stack(
              children: [
                ClipRRect(
                  borderRadius: BorderRadius.circular(20),
                  child: Image.asset(
                    product.photo,
                    width: 230,
                    height: 275,
                    fit: BoxFit.fitHeight,
                  ),
                ),
                Positioned(
                  top: 10,
                  right: 10,
                  child: Container(
                    padding: const EdgeInsets.all(8),
                    decoration: BoxDecoration(
                      color: const Color.fromARGB(94, 158, 158, 158),
                      borderRadius: BorderRadius.circular(50),
                    ),
                    child: SvgPicture.asset(
                      'assets/icons/favorite.svg',
                      color: Colors.white,
                      width: 24,
                      height: 24,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 15),
            Productdescription(product: product),
          ],
        ),
      ),
    );
  }
}

class Productdescription extends StatelessWidget {
  final Product product;
  const Productdescription({
    required this.product,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            customsubText(text: product.store),
            Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                SvgPicture.asset(
                  'assets/icons/ratingstar.svg',
                  width: 16,
                  height: 16,
                  color: Colors.yellow,
                ),
                const SizedBox(width: 5),
                customText(text: product.rating.toString()),
                const SizedBox(width: 5),
                customsubText(
                  text: '(${product.ratersNum})',
                ),
              ],
            ),
          ],
        ),
        const SizedBox(height: 10),
        customText(
          text: product.name,
        ),
        const SizedBox(height: 10),
        product.discount != null
            ? Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  customsubText(
                    text: product.discount.toString(),
                  ),
                  const SizedBox(width: 10),
                  customText(
                    text: product.price.toString(),
                  ),
                ],
              )
            : customText(
                text: product.price.toString(),
              ),
      ],
    );
  }
}
