import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';

class Link {
  final SvgPicture icon;
  final String name;

  Link({required this.icon, required this.name});
}

SvgPicture icon(String asset) {
  return SvgPicture.asset(
    asset,
    width: 28,
    height: 28,
    color: Color.fromARGB(255, 250, 137, 7),
  );
}

List<Link> links = [
  Link(icon: icon('assets/icons/home.svg'), name: 'Home'),
  Link(icon: icon('assets/icons/market.svg'), name: 'Market'),
  Link(icon: icon('assets/icons/about.svg'), name: 'About'),
  Link(icon: icon('assets/icons/accountInfo.svg'), name: 'Account')
];
