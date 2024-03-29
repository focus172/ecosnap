import React, { useState } from 'react';
import { StyleSheet, View } from 'react-native';

import VeiwPort from './components/VeiwPort';
import Explain from './components/Explain';
import Bar from './components//Bar';
import SearchBar from './components/SearchBar';

export default function App() {
  const [data, setData] = useState(null);
  const [search, setSearch] = useState(null);
  const [loading, setLoading] = useState(null);

  if (data != null) {
    return <View style={styles.container}>
      <Explain data={data}></Explain>
      <Bar setData={setData} setSearch={setSearch}></Bar>
    </View>

  } else if (search != null) {
    return <View style={styles.container}>
      <SearchBar search={search} setSearch={setSearch} setData={setData} setLoading={setLoading}></SearchBar>
      <Bar setData={setData} setSearch={setSearch}></Bar>
    </View>

  } else {
    return (
      <View style={styles.container}>
        <VeiwPort setData={setData} setLoading={setLoading} loading={loading}></VeiwPort>
        <Bar setData={setData} setSearch={setSearch}></Bar>
      </View>
    )
  }
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    paddingBottom: 10,
    paddingTop: 50,
    height: "90%",
    width: "100%"
  },
});
