import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import { Camera } from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import * as FileSystem from 'expo-file-system';
import Button from './Button';
import Icon from 'react-native-vector-icons/FontAwesome';
import Bar from './Bar';
import Description from './Description';

export default function Explain({ data }) {
    const resp = data["response"]

    // console.log(resp)
    const [ok, err] = [resp["ok"], resp["err"]]
    if (err != null) {
        return (
            <View style={styles.container}>
                <View>
                    <Text style={styles.header}>Error: {err}</Text>
                </View>
            </View>
        );
    }

    var name = "";
    var score = "";
    for (var i = 0, size = ok.length; i < size; i++) {
        name = ok[i]["name"];
        score = ok[i]["scores"];
    }
    console.log(name)
    console.log(score)

    return (
        <View style={styles.container}>
            <Text style={styles.header}>{name} Products</Text>
            <Text style={styles.score}>{score}</Text>
            <Description score={score}></Description>
        </View>
    );
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#fff',
        alignItems: 'center',
        width: "100%",
        height: "100%",
    },
    header: {
        fontSize: 30,
        fontWeight: 'bold',
    },
    score: {
        fontSize: 180,
        fontWeight: 'bold',
        color: 'red',
    },
});
