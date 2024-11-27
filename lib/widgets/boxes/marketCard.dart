import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:sweetmarket/services/models/products.dart';
import 'package:sweetmarket/widgets/texts/subText.dart';
import 'package:sweetmarket/widgets/texts/text.dart';

class Marketcard extends StatelessWidget {
  final Product product;
  const Marketcard({required this.product, super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(20),
      ),
      child: Padding(
        padding: const EdgeInsets.all(10.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Stack(
              children: [
                ClipRRect(
                  borderRadius: BorderRadius.circular(20),
                  child: Image.asset(
                    product.photo,
                    width: 160,
                    height: 200,
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
            Marketdescription(product: product),
          ],
        ),
      ),
    );
  }
}

class Marketdescription extends StatelessWidget {
  final Product product;
  const Marketdescription({
    required this.product,
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 160,
      height: 110,
      child: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            mainAxisSize: MainAxisSize.min,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              customsubText(
                text: product.store,
                color: Colors.black38,
              ),
              Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  SvgPicture.asset(
                    'assets/icons/ratingStar.svg',
                    width: 16,
                    height: 16,
                    color: Colors.yellow,
                  ),
                  const SizedBox(width: 5),
                  customText(
                    text: product.rating.toString(),
                    color: Colors.black,
                  ),
                ],
              ),
            ],
          ),
          const SizedBox(height: 10),
          SizedBox(
            height: 40,
            child: customText(
              text: product.name,
              color: Colors.black,
            ),
          ),
          const SizedBox(height: 10),
          product.discount != null
              ? Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    customsubText(
                      text: product.discount.toString(),
                      color: Colors.red,
                    ),
                    const SizedBox(width: 10),
                    customText(
                      text: product.price.toString(),
                      color: Colors.black45,
                    ),
                  ],
                )
              : customText(
                  text: product.price.toString(),
                  color: Colors.black45,
                ),
        ],
      ),
    );
  }
}
