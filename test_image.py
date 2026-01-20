import numpy as np
import feagi_data_processing as fdp




#TODO gitignore pyi

dimensions = (4, 5, 3)

empty_image: fdp.io_data.ImageFrame = fdp.io_data.ImageFrame(
    fdp.io_data.image_descriptors.ColorChannelLayout.RGB, #TODO change ColorChannelLayout to ColorColorChannelLayout
    fdp.io_data.image_descriptors.ColorSpace.Linear,
    (dimensions[0], dimensions[1])
)

numpy_2 = np.ones((4, 5, 3)).astype(np.float32)
empty_image = fdp.io_data.ImageFrame.from_array(
    numpy_2,
    fdp.io_data.image_descriptors.ColorSpace.Linear,
    fdp.io_data.image_descriptors.MemoryOrderLayout.HeightsWidthsChannels
)

empty_image_py = empty_image.copy_to_numpy_array()

sensor_cache = fdp.io_processing.cache.SensorCache()
cortical_type = fdp.genome.SensorCorticalType.ImageCameraCenter
cortical_grouping_index = 4
number_channels = 1
channel_dimensions = fdp.genome.SingleChannelDimensions(dimensions[0], dimensions[1], dimensions[2]) # TODO accept tuples
channel = 0
input_processors = [
    fdp.io_processing.processors.IdentityImageFrameProcessor(empty_image)
]
allow_sending_stale_data = True

sensor_cache.register_single_cortical_area(cortical_type, cortical_grouping_index, number_channels, channel_dimensions)
sensor_cache.register_single_channel(cortical_type, cortical_grouping_index, channel, input_processors, allow_sending_stale_data)


random_noise_image: fdp.io_data.ImageFrame = fdp.io_data.ImageFrame.from_array(
    np.random.rand(*dimensions).astype(np.float32),
    fdp.io_data.image_descriptors.ColorSpace.Linear,
    fdp.io_data.image_descriptors.MemoryOrderLayout.HeightsWidthsChannels #TODO why isnt it showing others
)

random_noise_image_py = random_noise_image.copy_to_numpy_array()

random_noise_image_py_darker = random_noise_image.copy_to_numpy_array()




#sensor_cache.update_value_by_channel(random_noise_image, cortical_type, cortical_grouping_index, channel)
neuron_output = sensor_cache.encode_to_neurons() #TODO why is this listed as nonexistant?
feagi_byte_structure = neuron_output.as_new_feagi_byte_structure()
byte = feagi_byte_structure.copy_out_as_byte_vector()
print(byte)
print(len(byte)) # should be 978
