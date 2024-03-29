import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import { Camera } from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import * as FileSystem from 'expo-file-system';
import Button from './Button';
import Icon from 'react-native-vector-icons/FontAwesome';
import Bar from './Bar';
import Loading from './Loading';

export default function VeiwPort({ setData, setLoading, loading }) {
  const [hasCameraPermission, setHasCameraPermission] = useState(null);
  const [image, setImage] = useState(null);
  const [type, setType] = useState(Camera.Constants.Type.back);
  const [flash, setFlash] = useState(Camera.Constants.FlashMode.off);
  const cameraRef = useRef(null);


  useEffect(() => {
    (async () => {
      MediaLibrary.requestPermissionsAsync();
      const cameraStatus = await Camera.requestCameraPermissionsAsync();
      setHasCameraPermission(cameraStatus.status === 'granted');
    })();
  }, []);

  const takePicture = async () => {
    if (cameraRef) {
      try {
        setLoading(loading => 'loading...')
        const data = await cameraRef.current.takePictureAsync();
        console.log(data);
        setImage(data.uri);
        const base64Image = await imageToBase64(data.uri);
        await sendImage(base64Image); // Sending the base64 image data as a POST request
      } catch (e) {
        console.log(e);
      }
    }
  };

  const imageToBase64 = async (uri) => {
    try {
      const file = await FileSystem.readAsStringAsync(uri, {
        encoding: FileSystem.EncodingType.Base64,
      });
      return file;
    } catch (error) {
      console.error('Error reading file:', error);
      return null;
    }
  };

  const sendImage = async (base64Image) => {
    try {
      const response = await fetch('http://172.31.164.78:6699/search', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ image: base64Image }),
      });

      setLoading(loading => null)
      if (response.ok) {
        console.log('Image sent successfully');
        const json = await response.json()
        setData(_data => json)
      } else {
        console.error('Failed to send image');
      }
    } catch (error) {
      console.error('Error sending image:', error);
    }
  };


  if (hasCameraPermission === false) {
    return <Text>No access to camera</Text>;
  }
  return (
    <View style={styles.container}>
      <Camera
        style={styles.camera}
        type={type}
        flashMode={flash}
        ref={cameraRef}
      >
        <View
          style={{
            width: '100%',
            height: 200,
            justifyContent: 'center',
            position: 'absolute',
            bottom: 0,
          }}
        >
          <Loading loading={loading}></Loading>
          <Button
            icon='circle'
            onPress={takePicture}
            size={100}
          ></Button>
        </View>
      </Camera>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    width: "130%",
    height: "100%",
  },
  camera: {
    flex: 1,
    // width: 400,
    borderRadius: 20,
    width: "100%",
    height: "38%",
  },
});
