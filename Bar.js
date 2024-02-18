import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import { Camera } from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import * as FileSystem from 'expo-file-system';
import Button from './Button';
import Icon from 'react-native-vector-icons/FontAwesome';

export default function Bar({ setData, setSearch }) {
    const goHome = async () => {
        // set the camera data to nothin to return home
        setData(data => null)
        setSearch(search => null)
    };
    const goSearch = async () => {
        setSearch(search => "")
        setData(data => null)
    };

    return <View
            style={{
                flexDirection: 'row',
                justifyContent: 'space-between',
                paddingHorizontal: 150,
            }}
        >
            <Icon.Button
                name='search'
                color={'black'}
                onPress={goSearch}
                backgroundColor={'white'}
                style={{
                    marginRight: 50,
                }}
            ></Icon.Button>
            <Icon.Button
                name='camera'
                color={'black'}
                onPress={goHome}
                backgroundColor={'white'}
                style={{
                    marginLeft: 50,
                }}
            ></Icon.Button>
        </View>;
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#fff',
        alignItems: 'center',
        paddingBottom: 50,
    },
    camera: {
        flex: 1,
        width: 500,
        borderRadius: 20,
    },
});
