// SearchBar.js
import React from "react";
import { StyleSheet, TextInput, View, Keyboard, Button } from "react-native";
import { Feather, Entypo } from "@expo/vector-icons";

export default function SearchBar({ search, setSearch, setData }) {
    const searchData = async () => {
        await seachDataName(search)
    };

    const seachDataName = async (name) => {
        try {
            const response = await fetch('http://172.31.164.78:6699/get/' + name, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
            });

            if (response.ok) {
                const json = await response.json()
                setData(data => json)
                // console.log(json);
            } else {
                console.error('Failed to send image');
            }
        } catch (error) {
            console.error('Error sending image:', error);
        }
    };
    return (
        <View style={styles.container}>
            <View
                style={styles.searchBar__clicked}
            >
                {/* search Icon */}
                <Feather
                    name="search"
                    size={20}
                    color="black"
                    style={{ marginLeft: 1 }}
                />
                {/* Input field */}
                <TextInput
                    style={styles.input}
                    placeholder="Search"
                    value={search}
                    onChangeText={setSearch}
                />
            </View>

            <Button title={"Done"} onPress={searchData}></Button>
        </View>
    );
};

// styles
const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#fff',
        alignItems: 'center',
        width: "100%",
        height: "100%",
    },
    searchBar__unclicked: {
        padding: 10,
        flexDirection: "row",
        width: "95%",
        backgroundColor: "#d9dbda",
        borderRadius: 15,
        alignItems: "center",
    },
    searchBar__clicked: {
        padding: 10,
        flexDirection: "row",
        width: "80%",
        backgroundColor: "#d9dbda",
        borderRadius: 15,
        alignItems: "center",
        justifyContent: "space-evenly",
    },
    input: {
        fontSize: 20,
        marginLeft: 10,
        width: "90%",
    },
});