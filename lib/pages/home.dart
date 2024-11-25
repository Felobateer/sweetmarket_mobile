import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:sweetmarket/services/models/products.dart';
import 'package:sweetmarket/widgets/boxes/appBar.dart';
import 'package:sweetmarket/widgets/boxes/categoryCard.dart';
import 'package:sweetmarket/services/models/categories.dart';
import 'package:sweetmarket/widgets/boxes/productCard.dart';
import 'package:sweetmarket/widgets/buttons/colorBtn.dart';
import 'package:sweetmarket/widgets/buttons/transparentBtn.dart';
import 'package:sweetmarket/widgets/texts/subTitle.dart';
import 'package:sweetmarket/widgets/texts/text.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: Appbar(
        left: Padding(
            padding: EdgeInsets.all(10.0),
            child: Image.asset('assets/imgs/sweetmarketLogo.png',
                width: 48, height: 48, fit: BoxFit.cover)),
        right: Container(
          padding: const EdgeInsets.symmetric(horizontal: 10),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Transparentbtn(
                content: 'Search',
                child: SvgPicture.asset(
                  'assets/icons/search.svg',
                  width: 24,
                  height: 24,
                  color: Colors.white,
                ),
                action: () => print('Search button clicked'),
              ),
              Transparentbtn(
                content: 'Cart',
                child: SvgPicture.asset(
                  'assets/icons/cart.svg',
                  width: 24,
                  height: 24,
                  color: Colors.white,
                ),
                action: () => print('Cart button clicked'),
              ),
            ],
          ),
        ),
      ),
      body: SingleChildScrollView(
        child: ConstrainedBox(
          constraints: BoxConstraints(
            minHeight: MediaQuery.of(context)
                .size
                .height, // Ensure it takes at least the full height
          ),
          child: IntrinsicHeight(
            child: Column(
              children: [
                HeroSection(),
                CategoryList(),
                Foryou(),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

class HeroSection extends StatelessWidget {
  const HeroSection({super.key});

  @override
  Widget build(BuildContext context) {
    return Container(
      alignment: Alignment.centerRight,
      height: 220,
      color: Colors.white,
      child: Stack(
        children: [
          Row(
            children: [
              // Empty container to hold the Image and the Column
              Expanded(
                child: Container(
                  alignment: Alignment.centerRight,
                  child: Image.asset(
                    'assets/imgs/hero.png',
                    width: 280,
                    height: 220,
                  ),
                ),
              ),
            ],
          ),
          Positioned(
              left: 20, // Adjust position from the right side
              top: 30, // Adjust position from the top side
              child: SizedBox(
                width: 175,
                child: Column(
                  children: [
                    customsubTitle(
                        text: 'All Your Favorite Snacks Gathered in One Place'),
                    const SizedBox(height: 20),
                    Colorbtn(
                      text: 'Shop Now',
                      color: Color(0xFFD97706),
                      action: () => print('Shop Now'),
                    ),
                  ],
                ),
              )),
        ],
      ),
    );
  }
}

class CategoryList extends StatelessWidget {
  const CategoryList({super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: [
        Padding(
            padding: EdgeInsets.all(10),
            child: Row(
              mainAxisSize: MainAxisSize.max,
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                customText(text: 'Shop By Category'),
                // const SizedBox(width: ),
                Transparentbtn(
                    content: 'show Categories',
                    action: () => print('categories clicked'),
                    child: customText(text: 'Show all'))
              ],
            )),
        Padding(
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
        )
      ],
    );
  }
}

class Foryou extends StatelessWidget {
  const Foryou({super.key});

  @override
  Widget build(BuildContext context) {
    return Padding(
        padding: EdgeInsets.all(20),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              children: [
                customsubTitle(text: 'For You'),
                const SizedBox(width: 10),
                SvgPicture.asset(
                  'assets/icons/rightArrow.svg',
                  width: 18,
                  height: 18,
                )
              ],
            ),
            SizedBox(
              height: 425,
              child: ListView.builder(
                scrollDirection: Axis.horizontal,
                itemCount: products.length,
                itemBuilder: (context, index) {
                  return Padding(
                      padding: const EdgeInsets.only(right: 20),
                      child: Productcard(product: products[index]));
                },
              ),
            ),
          ],
        ));
  }
}
