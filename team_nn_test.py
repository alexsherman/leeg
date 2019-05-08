from __future__ import absolute_import, division, print_function
import csv
import tensorflow as tf
from tensorflow import keras
import pandas as pd
import numpy as np
from spider_classes import *

def processCSV(match_csv="match_vectors.csv"):
    column_names = ["matchId", "winner"]
    column_names = ['winner']
    for key in champ_indexes.keys():
        column_names.append(key + 'BluePick')
    for key in champ_indexes.keys():
        column_names.append(key + 'RedPick')
    df = pd.read_csv(match_csv, names=column_names);
#    df = df.drop(['matchId'], axis=1)
    df['winner'] = df.apply(transform_team, axis=1)  
    print(df.head())

    dataset = df.copy()
    train_dataset = dataset.sample(frac=0.8,random_state=0)
    test_dataset = dataset.drop(train_dataset.index)
    train_labels = train_dataset.pop("winner")
    test_labels = test_dataset.pop('winner')
    model = keras.Sequential([
        keras.layers.Dense(294, activation=tf.nn.relu, input_shape=[len(train_dataset.keys())]),
        keras.layers.Dense(294, activation=tf.nn.relu),  
        keras.layers.Dense(1)
    ])
    #optimizer = keras.optimizers.SGD(lr=0.005, momentum=2.0, decay=0.0, nesterov=False)
    model.compile(optimizer="adam", 
        loss='binary_crossentropy',
        metrics=['accuracy', 'binary_crossentropy'])
    model.summary()
    example_batch = train_dataset[:10]
    example_result = model.predict(example_batch)
    print(example_result)
    model.fit(train_dataset, train_labels, epochs=100, validation_split = 0.2, verbose = 0)
    model.evaluate(test_dataset, test_labels)
   
def transform_team(row):
    if row['winner'] == 'blue':
        return 1
    if row['winner'] == 'red':
        return 0

if __name__ == '__main__':
    processCSV()
