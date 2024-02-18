import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef} from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import {Camera, CameraType} from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import Button from './button';

export default function App() {
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
    if(cameraRef) {
      try{
        const data = await cameraRef.current.takePictureAsync();
        console.log(data);
        setImage(data.uri);
      } catch(e){
        console.log(e);
      }
    }
  }

const saveImage = async () => {
  if(image){
    try {
      await MediaLibrary.createAssetAsync(image);
      alert('Saved!');
      setImage(null);
    } catch(e){
      console.log(e)
    }
  }
}
  if(hasCameraPermission === false) {
    return <Text>No access to camera</Text>
  }
  return (
    <View style={styles.container}>
      {!image ?
      <Camera
        style = {styles.camera}
        type={type}
        FlashMode={flash}
        ref={cameraRef}
        >
          <Text>hello</Text> 
        </Camera>
        :
        <Image source={{uri: image}} style={styles.camera}/>
      }
        <View>
          {image ?
          <View style={{
            flexDirection: 'row',
            justifyContent: 'space-between',
            paddingHorizontal: 50
          }}>
            <Button title={"Re-Take"} icon="retweet" onPress={() => setImage(null)}/>
            <Button title={"Save"} icon="check" onPress={saveImage}/>
          </View>
          :
            <Button title={'Take a Picture'} icon='camera' onPress={takePicture} />
          }
        </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    paddingBottom: 50
  },
  camera: {
    flex: 1,
    width: 500,
    borderRadius: 20,
  }
});
