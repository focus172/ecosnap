import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import { Camera } from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import * as FileSystem from 'expo-file-system';
import Icon from 'react-native-vector-icons/FontAwesome';

export default function Loading({ loading }) {
  if (loading != null) {
    return <Text style={{
        fontSize: 30,
        fontWeight: 'bold',
        left: 10
    }}>Loading...</Text>
  }
    return <View></View>
}
