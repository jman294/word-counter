class WordFrequencyCounter {
  Map<String, int> wordFreq (List<String> list) {
    var table = {};
    for (var word in list) {
      word = word.toLowerCase();
      if (table[word] == null) {
        table[word] = 0;
      }
      table[word] += 1;
    }

    return table;
  }
}
