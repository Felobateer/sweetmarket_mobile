import 'package:flutter/material.dart';

class customText extends StatelessWidget {
  final String text;
  const customText({required this.text, super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: TextStyle(
        fontSize: 16,
        fontFamily: 'Poppins',
        color: Colors.black,
        height: 1.1,
      ),
    );
  }
}
