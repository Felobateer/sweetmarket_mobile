import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

typedef Action = void Function();

class Transparentbtn extends StatelessWidget {
  final String content;
  final Action action;
  final Widget child;

  const Transparentbtn({
    required this.content,
    required this.action,
    required this.child, // This allows any type of child widget (e.g., SVG, Text)
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return ElevatedButton(
      onPressed: action,
      style: ElevatedButton.styleFrom(
        backgroundColor:
            Colors.transparent, // Make button background transparent
        elevation: 0, // Remove elevation to maintain flat appearance
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(30), // Rounded corners
        ),
        padding:
            const EdgeInsets.symmetric(vertical: 16.0), // Only vertical padding
      ),
      child: DefaultTextStyle(
        style: const TextStyle(
          color: Colors.white, // Ensure the text color is white
          fontSize: 16.0, // Font size for text
        ),
        child: child, // Allow the child to be any widget, like an SVG or text
      ),
    );
  }
}
