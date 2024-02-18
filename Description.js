import { StatusBar } from 'expo-status-bar';
import React, { useState, useEffect, useRef } from 'react';
import { StyleSheet, Text, View, Image } from 'react-native';
import { Camera } from 'expo-camera';
import * as MediaLibrary from 'expo-media-library';
import * as FileSystem from 'expo-file-system';
import Button from './Button';
import Icon from 'react-native-vector-icons/FontAwesome';

export default function Description({ score }) {
    if (score >= 0 && score <= 5) {
        text = `Brands with scores in the 0-5% range generally disclose minimal information, if any, primarily focusing on hiring practices or local community initiatives. In these cases, the information shared is often mandated by law, such as modern slavery statements or gender pay gap reports. Legislation mandating transparency proves to be a crucial instrument in encouraging non-disclosing brands to share information.`;
    } else if (score >= 6 && score <= 10) {
        text = `Brands with scores in the 6-10% range generally share limited policies related to their employees and suppliers. Brands nearing the 10% mark are more likely to disclose a basic supplier code of conduct. They provide some insights into their procedures and a limited overview of their supplier assessment processes.`;
    } else if (score >= 11 && score <= 20) {
        text = `Brands with scores in the 11%-20% range generally publish numerous policies regarding employees and suppliers, along with some procedures and details on their supplier evaluation and correction practices. These brands typically do not provide supplier lists and offer limited, if any, information on key issues such as purchasing practices, equality in gender and race, sustainable sourcing, overconsumption, waste management, water and chemical use, and climate change and biodiversity.`;
    } else if (score >= 21 && score <= 30) {
        text = `Brands with scores in the 21-30% range generally provide more detailed information regarding their policies, procedures, governance, as well as social and environmental goals. Brands may be releasing minimal information regarding their manufacturers, and do not include more details such as the number of workers, or any specific practices or standards adhered to by these factories. These brands will not be disclosing information on all Spotlight Issues but may touch upon a few.`;
    } else if (score >= 31 && score <= 40) {
        text = `Brands with scores in the 31-40% range generally disclose their first manufacturers and detailed information on their policies, procedures, social and environmental objectives, governance, and supplier evaluation and correction methods. Brands more likely to disclose some information regarding issues such as carbon emissions, gender equality, sustainable sourcing and materials, and energy use.`;
    } else if (score >= 41 && score <= 50) {
        text = `Brands with scores in the 41-50% range generally publish more in-depth supplier lists, including both manufacturers and processing facilities. Shares comprehensive details about their policies, governance, social, and environmental objectives. Insights into their supplier evaluation and remediation efforts, including some assessment findings. Increased engagement in addressing key Spotlight Issues.`;
    } else if (score >= 51 && score <= 60) {
        text = `Brands with scores in the 51-60% zone provide comprehensive disclosures, including detailed supplier lists. Comprehensive disclosures with detailed supplier lists. Address most human rights, environmental policies, and social goals. Provide insights from supplier assessments. Tackles key issues: carbon emissions, gender equality, sustainable sourcing, energy use, waste management, etc.`;
    } else if (score >= 61 && score <= 70) {
        text = `Brands with scores in the 61-70% range disclose comprehensive information, including detailed lists covering manufacturers, processing facilities, and select raw material suppliers. They also tackle key issues like racial equality, forced labor, overconsumption, deforestation, purchasing practices, and workers' rights, including unionization and collective bargaining.`;
    } else if (score >= 71 && score <= 80) {
        text = `Brands with scores in the 71-80% range provide detailed supplier lists for manufacturers, processing facilities, and raw material suppliers. They also offer detailed insights into due diligence processes, assessments, and remediation findings. While they share more comprehensive data on key issues, they may still lack significant disclosures on outcomes and impacts compared to other brands in the index.`;
    } else if (score >= 81 && score <= 100) {
        text = `Brands with scores in the 81-100% range disclose all information and provide very detailed insights into supplier assessments and remediation findings for specific facilities. They offer supplier lists covering at least 95% of all suppliers at manufacturing and processing levels, along with extensive raw material supplier lists. These brands are transparent about their social and environmental impacts, including their use of sustainable materials, gender breakdown in job roles, purchasing practices, and progress toward living wages in their supply chain. They disclose carbon emissions, renewable energy usage, and water footprint across their operations and supply chains.`;
    } else {
        text = 'Invalid score.';
    }

    return <Text>{text}</Text>;
}