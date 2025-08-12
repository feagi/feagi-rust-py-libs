"""Type stubs for feagi_data_processing module."""

from typing import Any

# Genome module
class genome:
    """Genomic structures module containing cortical types and dimensions."""
    
    class CorticalID:
        """Unique identifier for cortical areas in FEAGI.
        
        CorticalIDs are used to identify different brain regions/areas within
        the FEAGI neural system. They can represent different types of cortical
        areas including sensory, motor, core, memory, and custom areas.
        """
        
        @staticmethod
        def new_custom_cortical_area_id(desired_id_string: str) -> 'CorticalID':
            """Create a new custom cortical area ID.
            
            Args:
                desired_id_string: String identifier for the custom area
                
            Returns:
                New CorticalID for a custom cortical area
                
            Raises:
                ValueError: If the ID string is invalid
            """
            ...
        
        @staticmethod
        def new_memory_cortical_area_id(desired_id_string: str) -> 'CorticalID':
            """Create a new memory cortical area ID.
            
            Args:
                desired_id_string: String identifier for the memory area
                
            Returns:
                New CorticalID for a memory cortical area
                
            Raises:
                ValueError: If the ID string is invalid
            """
            ...
        
        @staticmethod
        def new_core_cortical_area_id(core_cortical_type: 'CoreCorticalType') -> 'CorticalID':
            """Create a new core cortical area ID.
            
            Args:
                core_cortical_type: Type of core cortical area (Death, Power)
                
            Returns:
                New CorticalID for a core cortical area
                
            Raises:
                ValueError: If the core type is invalid
            """
            ...
        
        @staticmethod
        def new_sensor_cortical_area_id(sensor_cortical_type: Any, input_index: Any) -> 'CorticalID':
            """Create a new sensor cortical area ID.
            
            Args:
                sensor_cortical_type: Type of sensor cortical area
                input_index: CorticalGroupingIndex or integer for the input
                
            Returns:
                New CorticalID for a sensor cortical area
                
            Raises:
                ValueError: If parameters are invalid
            """
            ...
        
        @staticmethod
        def create_ordered_cortical_areas_for_segmented_vision(camera_index: Any) -> list['CorticalID']:
            """Create cortical areas for segmented vision (9 segments).
            
            Args:
                camera_index: CorticalGroupingIndex or integer for the camera
                
            Returns:
                List of 9 CorticalIDs for segmented vision areas
                
            Raises:
                ValueError: If camera index is invalid
            """
            ...
        
        @staticmethod
        def try_new_from_bytes(bytes: bytes) -> 'CorticalID':
            """Create CorticalID from byte array.
            
            Args:
                bytes: Byte array representing the cortical ID
                
            Returns:
                New CorticalID from bytes
                
            Raises:
                ValueError: If bytes are invalid
            """
            ...
        
        @staticmethod
        def try_new_from_string(string: str) -> 'CorticalID':
            """Create CorticalID from string.
            
            Args:
                string: String representation of cortical ID
                
            Returns:
                New CorticalID from string
                
            Raises:
                ValueError: If string is invalid
            """
            ...
        
        @staticmethod
        def try_from_cortical_type(cortical_type: 'CorticalType', io_cortical_index: Any) -> 'CorticalID':
            """Create CorticalID from cortical type and index.
            
            Args:
                cortical_type: Type of cortical area
                io_cortical_index: CorticalGroupingIndex or integer
                
            Returns:
                New CorticalID from type and index
                
            Raises:
                ValueError: If parameters are invalid
            """
            ...
        
        def as_bytes(self) -> bytes:
            """Get byte representation of the cortical ID.
            
            Returns:
                Byte array representation
            """
            ...
        
        def as_ascii_string(self) -> str:
            """Get ASCII string representation of the cortical ID.
            
            Returns:
                ASCII string representation
            """
            ...
        
        def get_cortical_type(self) -> 'CorticalType':
            """Get the cortical type of this ID.
            
            Returns:
                CorticalType representing the area type
            """
            ...
        
        @property
        def CORTICAL_ID_LENGTH(self) -> int:
            """Length of cortical ID in characters."""
            ...
        
        @property
        def NUMBER_OF_BYTES(self) -> int:
            """Number of bytes in cortical ID."""
            ...
    
    class CorticalType:
        """Represents different types of cortical areas in FEAGI."""
        
        def __init__(self, core_type: 'CoreCorticalType') -> None:
            """Create CorticalType from core type.
            
            Args:
                core_type: Core cortical type (Death, Power)
            """
            ...
        
        @staticmethod
        def new_core(core_type: 'CoreCorticalType') -> 'CorticalType':
            """Create core cortical type.
            
            Args:
                core_type: Core cortical type (Death, Power)
                
            Returns:
                CorticalType for core areas
            """
            ...
        
        @staticmethod
        def new_sensor(sensor_type: Any) -> 'CorticalType':
            """Create sensor cortical type.
            
            Args:
                sensor_type: Sensor cortical type
                
            Returns:
                CorticalType for sensor areas
            """
            ...
        
        @staticmethod
        def new_custom() -> 'CorticalType':
            """Create custom cortical type.
            
            Returns:
                CorticalType for custom areas
            """
            ...
        
        @staticmethod
        def new_memory() -> 'CorticalType':
            """Create memory cortical type.
            
            Returns:
                CorticalType for memory areas
            """
            ...
        
        def get_type_variant(self) -> 'CorticalTypeVariant':
            """Get the variant of this cortical type.
            
            Returns:
                CorticalTypeVariant enum value
            """
            ...
    
    class CorticalTypeVariant:
        """Enum representing variants of cortical types."""
        Custom: 'CorticalTypeVariant'
        Memory: 'CorticalTypeVariant'
        Core: 'CorticalTypeVariant'
        Sensory: 'CorticalTypeVariant'
        Motor: 'CorticalTypeVariant'
    
    class CoreCorticalType:
        """Enum representing core cortical area types."""
        Death: 'CoreCorticalType'
        Power: 'CoreCorticalType'
    
    class CorticalGroupingIndex:
        """Index for grouping cortical areas."""
        
        def __init__(self, index: int) -> None:
            """Create new cortical grouping index.
            
            Args:
                index: Index value (0-255)
                
            Raises:
                ValueError: If index is out of range
            """
            ...
    
    class CorticalIOChannelIndex:
        """Index for IO channels within cortical areas."""
        
        def __init__(self, index: int) -> None:
            """Create new cortical IO channel index.
            
            Args:
                index: Channel index value
                
            Raises:
                ValueError: If index is out of range
            """
            ...
    
    class SingleChannelDimensions:
        """Dimensions for a single cortical channel."""
        
        def __init__(self, x: int, y: int, z: int) -> None:
            """Create new channel dimensions.
            
            Args:
                x: Width dimension
                y: Height dimension
                z: Depth dimension
                
            Raises:
                ValueError: If dimensions are invalid
            """
            ...

# IO Data module
class io_data:
    """IO data types and variants module."""
    
    class IOTypeVariant:
        """Type identifier for different kinds of IO data without values."""
        ...
    
    class ImageFrame:
        """Single image/frame data structure for vision processing."""
        ...
    
    class SegmentedImageFrame:
        """Multi-segment image frame for peripheral vision simulation."""
        ...
    
    class image_descriptors:
        """Image processing descriptors and configuration types."""
        
        class ImageFrameProperties:
            """Properties describing an image frame's format and dimensions."""
            
            def __init__(self, xy_resolution: tuple[int, int], color_space: 'ColorSpace', color_channel_layout: 'ChannelLayout') -> None:
                """Create new image frame properties.
                
                Args:
                    xy_resolution: Image dimensions as (width, height)
                    color_space: Color space (Linear or Gamma)
                    color_channel_layout: Channel layout (Grayscale, RGB, RGBA, etc.)
                    
                Raises:
                    ValueError: If resolution is invalid (zero dimensions)
                """
                ...
            
            @property
            def expected_xy_resolution(self) -> tuple[int, int]:
                """Expected image resolution as (width, height)."""
                ...
            
            @property
            def expected_color_space(self) -> 'ColorSpace':
                """Expected color space (Linear or Gamma)."""
                ...
            
            @property
            def expected_channel_layout(self) -> 'ChannelLayout':
                """Expected channel layout (Grayscale, RGB, RGBA, etc.)."""
                ...
        
        class CornerPoints:
            """Defines rectangular regions within images for cropping operations."""
            
            def __init__(self, lower_left: tuple[int, int], upper_right: tuple[int, int]) -> None:
                """Create corner points from row-major coordinates (origin top-left).
                
                Args:
                    lower_left: Lower-left corner as (row, col)
                    upper_right: Upper-right corner as (row, col)
                    
                Raises:
                    ValueError: If coordinates are invalid
                """
                ...
            
            @staticmethod
            def new_from_cartesian_where_origin_bottom_left(
                lower_left: tuple[int, int], 
                upper_right: tuple[int, int], 
                total_resolution_width_height: tuple[int, int]
            ) -> 'CornerPoints':
                """Create corner points from Cartesian coordinates (origin bottom-left).
                
                Args:
                    lower_left: Lower-left corner as (x, y)
                    upper_right: Upper-right corner as (x, y)
                    total_resolution_width_height: Total image size as (width, height)
                    
                Returns:
                    CornerPoints for the specified region
                    
                Raises:
                    ValueError: If coordinates are invalid or outside image bounds
                """
                ...
            
            def does_fit_in_frame_of_width_height(self, source_total_resolution: tuple[int, int]) -> bool:
                """Check if these corner points fit within the given frame size.
                
                Args:
                    source_total_resolution: Frame size as (width, height)
                    
                Returns:
                    True if the corner points fit within the frame
                """
                ...
            
            def enclosed_area_width_height(self) -> tuple[int, int]:
                """Get the dimensions of the enclosed rectangular area.
                
                Returns:
                    Area dimensions as (width, height)
                """
                ...
            
            @property
            def lower_right_row_major(self) -> tuple[int, int]:
                """Lower-right corner in row-major coordinates."""
                ...
            
            @property
            def upper_left_row_major(self) -> tuple[int, int]:
                """Upper-left corner in row-major coordinates."""
                ...
            
            @property
            def lower_left_row_major(self) -> tuple[int, int]:
                """Lower-left corner in row-major coordinates."""
                ...
            
            @property
            def upper_right_row_major(self) -> tuple[int, int]:
                """Upper-right corner in row-major coordinates."""
                ...
        
        class ColorSpace:
            """Enum representing different color spaces."""
            Linear: 'ColorSpace'
            Gamma: 'ColorSpace'
        
        class ChannelLayout:
            """Enum representing different channel layouts."""
            GrayScale: 'ChannelLayout'
            RG: 'ChannelLayout'
            RGB: 'ChannelLayout'
            RGBA: 'ChannelLayout'
        
        class MemoryOrderLayout:
            """Enum representing different memory ordering layouts."""
            HeightsWidthsChannels: 'MemoryOrderLayout'
            WidthsChannelsHeights: 'MemoryOrderLayout'
        
        class SegmentedFrameCenterProperties:
            """Properties for the center region of segmented vision frames."""
            ...
        
        class SegmentedFrameTargetResolutions:
            """Target resolutions for all segments in segmented vision frames."""
            ...

# IO Processing module
class io_processing:
    """IO processing utilities module."""
    
    class bytes:
        """Byte structure handling module."""
        class FeagiByteStructure:
            """FEAGI byte structure for efficient data serialization."""
            ...
    
    class processors:
        """Data processing modules."""
        
        class LinearAverageRollingWindowProcessor:
            """Rolling window processor that computes linear averages."""
            ...
        
        class IdentityFloatProcessor:
            """Identity processor for float values (passes through unchanged)."""
            
            def __init__(self, initial_value: float) -> None:
                """Create new identity float processor.
                
                Args:
                    initial_value: Initial cached value
                    
                Raises:
                    ValueError: If initial value is invalid
                """
                ...
        
        class IdentityImageFrameProcessor:
            """Identity processor for image frames (passes through unchanged)."""
            
            def __init__(self, initial_image: Any) -> None:
                """Create new identity image frame processor.
                
                Args:
                    initial_image: Initial cached image frame
                    
                Raises:
                    ValueError: If initial image is invalid
                """
                ...
        
        class LinearScaleTo0And1:
            """Linear scaling processor that maps input range to [0, 1]."""
            ...
        
        class LinearScaleToM1And1:
            """Linear scaling processor that maps input range to [-1, 1]."""
            ...
    
    class cache:
        """Caching mechanisms for sensors."""
        
        class SensorCache:
            """Cache for sensor data with processing pipelines."""
            
            def __init__(self) -> None:
                """Create new empty sensor cache."""
                ...
            
            def register_single_cortical_area(
                self, 
                cortical_sensor_type: Any, 
                cortical_grouping_index: Any, 
                number_supported_channels: int, 
                channel_dimensions: Any
            ) -> None:
                """Register a single cortical area for sensor data.
                
                Args:
                    cortical_sensor_type: Type of sensor cortical area
                    cortical_grouping_index: Grouping index for the area
                    number_supported_channels: Number of channels supported
                    channel_dimensions: Dimensions for each channel
                    
                Raises:
                    ValueError: If parameters are invalid
                """
                ...
            
            def register_single_channel(
                self,
                cortical_sensor_type: Any,
                cortical_grouping_index: Any,
                channel: Any,
                sensory_processors: list[Any],
                should_sensor_allow_sending_stale_data: bool
            ) -> None:
                """Register a single channel within a cortical area.
                
                Args:
                    cortical_sensor_type: Type of sensor cortical area
                    cortical_grouping_index: Grouping index for the area
                    channel: Channel index within the area
                    sensory_processors: List of processors for this channel
                    should_sensor_allow_sending_stale_data: Whether to allow stale data
                    
                Raises:
                    ValueError: If parameters are invalid
                """
                ...
            
            def update_value_by_channel(
                self,
                value: Any,
                cortical_sensor_type: Any,
                cortical_grouping_index: Any,
                channel: Any
            ) -> None:
                """Update sensor value for a specific channel.
                
                Args:
                    value: New sensor value
                    cortical_sensor_type: Type of sensor cortical area
                    cortical_grouping_index: Grouping index for the area
                    channel: Channel index within the area
                    
                Raises:
                    ValueError: If parameters are invalid
                """
                ...
            
            def export_as_cortical_mapped_xyzp_neuron_data(self) -> Any:
                """Export all cached sensor data as cortical mapped neuron data.
                
                Returns:
                    CorticalMappedXYZPNeuronData containing all sensor data
                    
                Raises:
                    ValueError: If export fails
                """
                ...

# Neuron Data module
class neuron_data:
    """Neuron data structures and utilities module."""
    
    class xyzp:
        """XYZP neuron data handling module."""
        
        class CorticalMappedXYZPNeuronData:
            """HashMap-like structure mapping CorticalID to NeuronXYZPArrays.
            
            This class represents a collection of neural data organized by cortical areas.
            Each cortical area (identified by a CorticalID) maps to an array of neurons
            with XYZP coordinates (x, y, z positions and power values).
            
            Extends PyFeagiByteStructureCompatible for serialization support.
            """
            
            def __init__(self) -> None:
                """Create a new empty cortical mapped neuron data structure."""
                ...
            
            @staticmethod
            def new_with_capacity(capacity: int) -> 'CorticalMappedXYZPNeuronData':
                """Create a new structure with pre-allocated capacity.
                
                Args:
                    capacity: Initial capacity for the number of cortical areas
                    
                Returns:
                    New instance with reserved capacity
                """
                ...
            
            @staticmethod
            def new_from_feagi_byte_structure(byte_structure: Any) -> 'CorticalMappedXYZPNeuronData':
                """Create from a FEAGI byte structure.
                
                Args:
                    byte_structure: Source byte structure to deserialize from
                    
                Returns:
                    New instance created from byte structure
                    
                Raises:
                    ValueError: If byte structure is invalid or incompatible
                """
                ...
            
            def len(self) -> int:
                """Get the number of cortical areas in the mapping.
                
                Returns:
                    Number of cortical areas currently stored
                """
                ...
            
            def is_empty(self) -> bool:
                """Check if the mapping contains any cortical areas.
                
                Returns:
                    True if no cortical areas are stored, False otherwise
                """
                ...
            
            def capacity(self) -> int:
                """Get the current capacity of the internal HashMap.
                
                Returns:
                    Number of cortical areas that can be stored without reallocation
                """
                ...
            
            def reserve(self, additional_capacity: int) -> None:
                """Reserve additional capacity for more cortical areas.
                
                Args:
                    additional_capacity: Additional number of cortical areas to reserve space for
                """
                ...
            
            def shrink_to_fit(self) -> None:
                """Shrink the internal capacity to fit the current number of cortical areas."""
                ...
            
            def get_neurons_of(self, cortical_id: Any) -> 'NeuronXYZPArrays | None':
                """Get the neuron arrays for a specific cortical area.
                
                Args:
                    cortical_id: The cortical ID to look up
                    
                Returns:
                    Neuron arrays for the cortical area, or None if not found
                """
                ...
            
            def contains_cortical_id(self, cortical_id: Any) -> bool:
                """Check if a cortical area exists in the mapping.
                
                Args:
                    cortical_id: The cortical ID to check for
                    
                Returns:
                    True if the cortical area exists, False otherwise
                """
                ...
            
            def contains(self, cortical_id: Any) -> bool:
                """Alias for contains_cortical_id for consistency.
                
                Args:
                    cortical_id: The cortical ID to check for
                    
                Returns:
                    True if the cortical area exists, False otherwise
                """
                ...
            
            def insert(self, cortical_id: Any, data: 'NeuronXYZPArrays') -> 'NeuronXYZPArrays | None':
                """Insert or update neuron data for a cortical area.
                
                Args:
                    cortical_id: The cortical ID to associate with the data
                    data: The neuron arrays to store
                    
                Returns:
                    Previous neuron arrays if the cortical area existed, None otherwise
                """
                ...
            
            def remove(self, cortical_id: Any) -> 'NeuronXYZPArrays | None':
                """Remove and return neuron data for a cortical area.
                
                Args:
                    cortical_id: The cortical ID to remove
                    
                Returns:
                    Removed neuron arrays if found, None otherwise
                """
                ...
            
            def clear(self) -> None:
                """Remove all cortical areas and their neuron data."""
                ...
            
            def keys(self) -> Any:
                """Get an iterator over all cortical IDs.
                
                Returns:
                    Iterator yielding PyCorticalID objects
                """
                ...
            
            def values(self) -> Any:
                """Get an iterator over all neuron arrays.
                
                Returns:
                    Iterator yielding PyNeuronXYZPArrays objects
                """
                ...
            
            def iter_full(self) -> Any:
                """Get an iterator over cortical IDs and their numpy arrays.
                
                Returns:
                    Iterator yielding tuples of (cortical_id_string, (x_array, y_array, z_array, p_array))
                """
                ...
            
            def __iter__(self) -> Any:
                """Enable Python iteration over (cortical_id, neuron_arrays) pairs.
                
                Returns:
                    Iterator yielding (PyCorticalID, PyNeuronXYZPArrays) tuples
                """
                ...
            
            # Inherited from PyFeagiByteStructureCompatible
            @property
            def byte_structure_type(self) -> Any:
                """Get the byte structure type identifier."""
                ...
            
            @property
            def byte_structure_version(self) -> int:
                """Get the byte structure version number."""
                ...
            
            @property
            def max_number_bytes_needed(self) -> int:
                """Get the maximum number of bytes needed for serialization."""
                ...
            
            def as_new_feagi_byte_structure(self) -> Any:
                """Serialize this structure to a FEAGI byte structure.
                
                Returns:
                    New PyFeagiByteStructure containing the serialized data
                    
                Raises:
                    ValueError: If serialization fails
                """
                ...
        
        class NeuronXYZPArrays:
            """Parallel arrays storing neuron data with XYZP coordinates.
            
            This class stores neuron data in separate arrays for X, Y, Z coordinates
            and P (power/activation) values. This structure is optimized for
            performance and memory efficiency when processing large numbers of neurons.
            """
            
            def __init__(self) -> None:
                """Create new empty neuron arrays."""
                ...
            
            @staticmethod
            def new_from_resolution(resolution: tuple[int, int, int]) -> 'NeuronXYZPArrays':
                """Create arrays based on a 3D resolution.
                
                Args:
                    resolution: Tuple of (width, height, depth) dimensions
                    
                Returns:
                    New neuron arrays sized for the given resolution
                """
                ...
            
            @staticmethod
            def new_from_numpy(x: Any, y: Any, z: Any, p: Any) -> 'NeuronXYZPArrays':
                """Create arrays from existing numpy arrays.
                
                Args:
                    x: Numpy array of X coordinates (u32)
                    y: Numpy array of Y coordinates (u32)
                    z: Numpy array of Z coordinates (u32)
                    p: Numpy array of power values (f32)
                    
                Returns:
                    New neuron arrays created from the input arrays
                    
                Raises:
                    ValueError: If arrays have mismatched lengths or invalid data
                """
                ...
            
            @staticmethod
            def with_capacity(number_of_neurons_initial: int) -> 'NeuronXYZPArrays':
                """Create arrays with pre-allocated capacity.
                
                Args:
                    number_of_neurons_initial: Initial capacity for number of neurons
                    
                Returns:
                    New neuron arrays with reserved capacity
                """
                ...
            
            def capacity(self) -> int:
                """Get the current capacity of the arrays.
                
                Returns:
                    Number of neurons that can be stored without reallocation
                """
                ...
            
            def spare_capacity(self) -> int:
                """Get the unused capacity of the arrays.
                
                Returns:
                    Number of additional neurons that can be stored without reallocation
                """
                ...
            
            def len(self) -> int:
                """Get the number of neurons currently stored.
                
                Returns:
                    Current number of neurons in the arrays
                """
                ...
            
            def is_empty(self) -> bool:
                """Check if the arrays contain any neurons.
                
                Returns:
                    True if no neurons are stored, False otherwise
                """
                ...
            
            def shrink_to_fit(self) -> None:
                """Shrink the arrays' capacity to fit the current number of neurons."""
                ...
            
            def ensure_capacity(self, number_of_neurons_total: int) -> None:
                """Ensure the arrays can hold at least the specified number of neurons.
                
                Args:
                    number_of_neurons_total: Minimum total capacity required
                """
                ...
            
            def reserve(self, additional_neuron_count: int) -> None:
                """Reserve additional capacity for more neurons.
                
                Args:
                    additional_neuron_count: Additional number of neurons to reserve space for
                """
                ...
            
            def push(self, new_neuron: 'NeuronXYZP') -> None:
                """Add a new neuron to the end of the arrays.
                
                Args:
                    new_neuron: The neuron to add
                """
                ...
            
            def get(self, index: int) -> 'NeuronXYZP':
                """Get a neuron at a specific index.
                
                Args:
                    index: Zero-based index of the neuron to retrieve
                    
                Returns:
                    The neuron at the specified index
                    
                Raises:
                    ValueError: If index is out of bounds
                """
                ...
            
            def pop(self) -> 'NeuronXYZP':
                """Remove and return the last neuron from the arrays.
                
                Returns:
                    The removed neuron
                    
                Raises:
                    ValueError: If arrays are empty
                """
                ...
            
            def clear(self) -> None:
                """Remove all neurons from the arrays while preserving capacity."""
                ...
            
            def copy_as_neuron_xyzp_vec(self) -> list['NeuronXYZP']:
                """Copy all neurons as a list of PyNeuronXYZP objects.
                
                Returns:
                    List containing copies of all neurons
                """
                ...
            
            def copy_as_tuple_of_numpy_arrays(self) -> tuple[Any, Any, Any, Any]:
                """Copy the arrays as numpy arrays.
                
                Returns:
                    Tuple of (x_array, y_array, z_array, p_array) as numpy arrays
                """
                ...
            
            def copy_as_tuple_of_numpy(self) -> tuple[Any, Any, Any, Any]:
                """Alias for copy_as_tuple_of_numpy_arrays.
                
                Returns:
                    Tuple of (x_array, y_array, z_array, p_array) as numpy arrays
                """
                ...
            
            def get_size_in_number_of_bytes(self) -> int:
                """Get the total size of the neuron data in bytes.
                
                Returns:
                    Total memory usage of all neuron data in bytes
                """
                ...
            
            def __iter__(self) -> Any:
                """Enable Python iteration over individual neurons.
                
                Returns:
                    Iterator yielding PyNeuronXYZP objects
                """
                ...
            
            def __str__(self) -> str:
                """String representation of the neuron arrays."""
                ...
        class NeuronXYZP:
            """Individual neuron with XYZP coordinates.
            
            Represents a single neuron with 3D spatial coordinates (X, Y, Z)
            and a power/activation value (P).
            """
            
            def __init__(self, x: int, y: int, z: int, p: float) -> None:
                """Create a new neuron with the specified coordinates and power.
                
                Args:
                    x: X coordinate (unsigned 32-bit integer)
                    y: Y coordinate (unsigned 32-bit integer)
                    z: Z coordinate (unsigned 32-bit integer)
                    p: Power/activation value (32-bit float)
                """
                ...
            
            def as_tuple(self) -> tuple[int, int, int, float]:
                """Get the neuron data as a tuple.
                
                Returns:
                    Tuple of (x, y, z, p) coordinates and power
                """
                ...
            
            def __str__(self) -> str:
                """String representation of the neuron."""
                ...