import 'package:flutter/material.dart';

class FormInput extends StatelessWidget {
  final String label;
  final String hint;

  const FormInput({required this.label, required this.hint, super.key});

  @override
  Widget build(BuildContext context) {
    return TextField(
      decoration: InputDecoration(
        labelText: label,
        hintText: hint,
        labelStyle: TextStyle(
          fontSize: 18,
          fontWeight: FontWeight.w400,
          fontFamily: 'Poppins',
        ),
        hintStyle: TextStyle(
          color: Colors.grey, // Optional: Change placeholder text color.
        ),
        border: OutlineInputBorder(
          borderRadius: BorderRadius.circular(15), // Rounded corners.
          borderSide: BorderSide(
            color: Color(0xFFD97706), // Amber border color.
            width: 2.0, // Border width.
          ),
        ),
        enabledBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(15.0),
          borderSide: BorderSide(
            color: Color(0xFFD97706),
            width: 2.0,
          ),
        ),
        focusedBorder: OutlineInputBorder(
          borderRadius: BorderRadius.circular(15.0),
          borderSide: BorderSide(
            color: Color(0xFFD97706),
            width: 2.5, // Thicker border when focused.
          ),
        ),
        contentPadding: EdgeInsets.symmetric(
          vertical: 16.0,
          horizontal: 12.0,
        ), // Padding inside the input field.
      ),
      style: TextStyle(
        fontSize: 16.0, // Input text size.
        color: Colors.black, // Input text color.
      ),
      cursorColor: Color(0xFFD97706),
    );
  }
}
