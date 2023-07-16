import React, { useEffect, useState } from 'react';
import dynamic from 'next/dynamic';
import {
  Flex,
  Box,
  useColorModeValue,
} from '@chakra-ui/react';
import { main } from '@popperjs/core';

const DynamicApexChart = dynamic(() => import('react-apexcharts'), { ssr: false });

export const Chart = () => {
  const maintenanceMarginRatio = 0.05 * 100;
  const initialMarginRatio = 0.08 * 100;
  const currentMarginRatio = 0.07 * 100

  const leftOver = 100 - maintenanceMarginRatio - initialMarginRatio - currentMarginRatio

  const seriesData = [maintenanceMarginRatio, currentMarginRatio, initialMarginRatio, leftOver]

  const options = {
    series: seriesData,
    chart: {
      width: 380,
      type: 'donut',
    },
    plotOptions: {
      pie: {
        startAngle: 0,
        endAngle: 360,
      },
    },
    legend: {
      show: false
    },
    dataLabels: {
      enabled: false,
    },
    fill: {
      colors: ['#F44336', '#E91E63', '#9C27B0', '#808080'],
    },
    responsive: [
      {
        breakpoint: 480,
        options: {
          chart: {
            width: 200,
          },
          legend: {
            position: 'bottom',
          },
        },
      },
    ],
    tooltip: {
      y: {
        formatter: function (value, { seriesIndex }) {
          if (seriesIndex === seriesData.length - 1) {
            return ''; // Hide tooltip for the "leftOver" data point
          }

          const seriesNames = [
            'Maintenance Margin Ratio',
            'Current Margin Ratio',
            'Initial Margin Ratio',
          ];

          return seriesNames[seriesIndex] + ': ' + value + '%';
        },
      },
      custom: function ({ series, seriesIndex }) {
        if (seriesIndex === seriesData.length - 1) {
          return ''; // Hide tooltip for the "leftOver" data point
        }

        const seriesNames = [
          'Maintenance Margin Ratio',
          'Current Margin Ratio',
          'Initial Margin Ratio',
        ];

        const seriesName = seriesNames[seriesIndex];
        const value = series[seriesIndex];
        if (seriesIndex === 1) {
          return `<div style="background-color: #81e6d9;color: #000;padding: 2px;">${seriesName}: 12%</div>`; // Customize the tooltip content
        }

        if (seriesIndex === 2) {
          return `<div style="background-color: #81e6d9;color: #000;padding: 2px;">${seriesName}: 20%</div>`; // Customize the tooltip content
        }
        return `<div style="background-color: #81e6d9;color: #000;padding: 2px;">${seriesName}: ${value}%</div>`;
      },
      series: {
        enabled: false,
      },
    },
  };

  const [chartOptions, setChartOptions] = useState(options);

  return (
    <Flex justifyContent="center" alignItems="center" flexDirection="row" mb="5">
      <Flex justifyContent="center" alignItems="center" flexDirection="column" py="7" >
        <h1>Margin Ratio</h1>
        {
          typeof window !== 'undefined' ? (
            <DynamicApexChart options={chartOptions} series={chartOptions.series} type="donut" width={380} />
          ) : null
        }
      </Flex>
      <Box border="1px solid"
        borderRadius="md"
        padding="5"
        borderColor={useColorModeValue('blackAlpha.200', 'whiteAlpha.200')}
      >
        <div>Liquidation Price: $6.00</div>
        <div>Oracle Price: $8.00</div>
      </Box>
    </Flex>
  );
};
