class WordSplitter {
  List<String> splitText (String str) {
    var words = [];
    var cWord = '';
    for (var i=0; i<str.length; i++) {
      if (isSplitChar(str[i])) {
        words.add(cWord);
        cWord = '';
      } else {
        cWord += str[i];
      }
    }

    return words;
  }

  bool isSplitChar (String char) {
    final splitChars = [' ', '.', '!', '?', '\n', ';', ',', '(', ')', '"', "'",
                        '“', '”', ':'];
    return splitChars.contains(char);
  }
}
