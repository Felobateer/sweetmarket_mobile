import 'package:flutter/material.dart';

class customsubText extends StatelessWidget {
  final String text;
  final Color color;
  const customsubText({required this.text, required this.color, super.key});

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: TextStyle(
        fontSize: 16,
        fontFamily: 'Poppins',
        color: color,
        fontWeight: FontWeight.w400,
      ),
    );
  }
}
