import 'package:flutter/material.dart';
import 'package:sweetmarket/widgets/boxes/appBar.dart';
import 'package:sweetmarket/widgets/texts/subText.dart';
import 'package:sweetmarket/widgets/texts/subTitle.dart';
import 'package:sweetmarket/widgets/texts/text.dart';

class About extends StatelessWidget {
  const About({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AboutAppbar(),
        body: Center(
            child: ListView.builder(
          scrollDirection: Axis.vertical,
          itemCount: aboutContents.length,
          itemBuilder: (context, index) {
            return Padding(
                padding: EdgeInsets.all(10),
                child: AboutParagraph(
                    title: aboutContents[index].title,
                    text: aboutContents[index].text));
          },
        )));
  }
}

class AboutAppbar extends StatelessWidget implements PreferredSizeWidget {
  const AboutAppbar({super.key});

  @override
  Widget build(BuildContext context) {
    return Appbar(
      left: Padding(
        padding: EdgeInsets.all(8.0),
        child: Image.asset(
          'assets/imgs/sweetmarketLogo.png',
          fit: BoxFit.cover,
        ),
      ),
      right: Expanded(
          child: Container(
        padding: EdgeInsets.all(10.0),
        child: customsubTitle(
          text: 'SWEETMARKET',
          color: Colors.white,
        ),
      )),
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(kToolbarHeight);
}

class AboutParagraph extends StatelessWidget {
  final String title;
  final String text;
  const AboutParagraph({required this.title, required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        customsubTitle(color: Colors.black87, text: title),
        const SizedBox(
          height: 10,
        ),
        customText(text: text, color: Colors.black54)
      ],
    );
  }
}

class aboutContent {
  final String title;
  final String text;

  const aboutContent({
    required this.title,
    required this.text,
  });
}

List<aboutContent> aboutContents = [
  aboutContent(
      title: 'Why us',
      text:
          'Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate?'),
  aboutContent(
      title: 'History',
      text:
          'Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate?Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate?'),
  aboutContent(
      title: 'Future',
      text:
          'Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate?Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate? Lorem ipsum dolor sit amet consectetur adipisicing elit. Consequuntur aperiam impedit rem facere sed repellat voluptatem deleniti similique consectetur cupiditate?'),
];
