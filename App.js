import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import { Camera } from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import * as FileSystem from 'expo-file-system';
import Button from './Button';
import VeiwPort from './VeiwPort';
import Explain from './Explain';
import Bar from './Bar';
import Icon from 'react-native-vector-icons/FontAwesome';

export default function App() {

  const [data, setData] = useState(null);
  const [search, setSearch] = useState(null);

  if (data == null) {
    return <VeiwPort setData={setData}></VeiwPort>
  } else if (search != null) {
    return <Text>Test</Text>
  } else {
    return <Explain data={data} setData={setData}></Explain>
  }
}